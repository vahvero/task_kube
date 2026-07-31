use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::{get, post},
};
use core::panic;
use diesel::prelude::*;
use diesel::{
    Connection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use futures_util::stream::{self, Stream};
use log::info;
use rusqlite::OpenFlags;
use schema::task::dsl::*;
use std::{env, error::Error};
use task_kube::{
    models::{NewTask, Task},
    schema,
    task_state::TaskState,
};
use tokio::sync::broadcast;
use tokio_stream::StreamExt as _;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<()>,
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in environment");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error establishing connection to {}", database_url))
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in environment");
        match rusqlite::Connection::open_with_flags(
            &database_url,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        ) {
            Ok(_) => {}
            Err(e) => panic!("Cannot established connection to {database_url} with {e}"),
        };
    }
    let mut conn = establish_connection();

    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let address = "0.0.0.0";
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => panic!("PORT not found in environment"),
    };
    let address = format!("{}:{}", address, port);
    info!("Running migrations");
    match run_migrations() {
        Ok(_) => {}
        Err(e) => panic!("Migrations failed with error: {:?}", e),
    }
    let (tx, _) = broadcast::channel(16);
    let app_state = AppState { tx };
    let app = Router::new()
        .route("/task", post(create_task).put(put_task).get(get_task))
        .route("/load-request", get(get_pending_task))
        .route("/tasks", get(sse_task_handler))
        .route("/reset", post(clear_all))
        .route("/healthcheck", get(async || StatusCode::OK))
        .fallback(fallback)
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Starting server on {}", address);
    axum::serve(listener, app).await.unwrap();
}

async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "Task server reached but cannot match route".into(),
    )
}

async fn clear_all(State(app_state): State<AppState>) -> StatusCode {
    let mut connection = establish_connection();
    let result = diesel::delete(task).execute(&mut connection);
    match result {
        Ok(count) => {
            info!("Reset {} rows from tasks", count);
            let _ = app_state.tx.send(());
            StatusCode::RESET_CONTENT
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn create_task(State(app_state): State<AppState>) -> StatusCode {
    info!("Create task called");
    use crate::schema::task;

    let mut connection = establish_connection();
    let new_task = NewTask {
        description: "New task",
        delay: 10,
        state: &TaskState::Pending.to_string(),
    };

    match diesel::insert_into(task::table)
        .values(&new_task)
        .execute(&mut connection)
    {
        Ok(_) => {
            let _ = app_state.tx.send(());
            StatusCode::CREATED
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_task(Path(task_id): Path<i32>) -> Result<Json<Task>, StatusCode> {
    info!("Get task {} requested", task_id);
    let mut connection = establish_connection();
    let item = task
        .find(task_id)
        .select(Task::as_select())
        .first(&mut connection)
        .optional();
    match item {
        Ok(Some(item)) => Ok(Json(item)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn put_task(State(app_state): State<AppState>, Json(_task): Json<Task>) -> StatusCode {
    info!("Put requested for id {} to state {}", _task.id, _task.state);
    let mut connection = establish_connection();

    let result = diesel::update(task)
        .filter(id.eq(_task.id))
        .set(&_task)
        .execute(&mut connection);

    match result {
        Ok(count) => {
            info!("Put executed with {} affected", count);
            let _ = app_state.tx.send(());
            StatusCode::OK
        }
        Err(_) => StatusCode::NOT_FOUND,
    }
}

async fn get_pending_task() -> Result<Json<Task>, StatusCode> {
    info!("Pending task requested");
    let mut connection = establish_connection();
    let item = task
        .filter(state.eq("PENDING"))
        .select(Task::as_select())
        .first(&mut connection)
        .optional();
    match item {
        Ok(Some(item)) => Ok(Json(item)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn sse_task_handler(
    State(app_state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let tasks = fetch_tasks().unwrap_or_else(|_| panic!("Error loading tasks from database"));
    let initial_json = serde_json::to_string(&tasks).unwrap();
    info!("Initiliasing stream");
    let initial_stream = stream::once(async move {
        Ok::<_, std::convert::Infallible>(Event::default().data(initial_json))
    });

    let update_stream = stream::unfold(app_state.tx.subscribe(), |mut rx| async move {
        match rx.recv().await {
            Ok(_) => {
                info!("Starting SSE fetch");
                let tasks =
                    fetch_tasks().unwrap_or_else(|_| panic!("Error loading tasks from database"));
                let json = serde_json::to_string(&tasks).unwrap();
                info!("Sending SSE {} rows of data", tasks.len());
                info!(
                    "{} tasks are in IN-PROGRESS state",
                    tasks
                        .into_iter()
                        .filter(|x| TaskState::InProgress == x.state)
                        .count()
                );
                Some((
                    Ok::<_, std::convert::Infallible>(Event::default().data(json)),
                    rx,
                ))
            }
            Err(broadcast::error::RecvError::Lagged(n)) => {
                info!("SSE client lagged behind by {} messages", n);
                let tasks =
                    fetch_tasks().unwrap_or_else(|_| panic!("Error loading tasks from database"));
                let json = serde_json::to_string(&tasks).unwrap();
                Some((
                    Ok::<_, std::convert::Infallible>(Event::default().data(json)),
                    rx,
                ))
            }
            Err(broadcast::error::RecvError::Closed) => None,
        }
    });

    let stream = initial_stream.chain(update_stream);
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

fn fetch_tasks() -> QueryResult<Vec<Task>> {
    let mut connection = establish_connection();
    task.select(Task::as_select()).load(&mut connection)
}
