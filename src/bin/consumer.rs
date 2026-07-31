/**
 * This is the "calculation" server which is auto
 * scaled up/down.
 *
 * */
use std::{
    env, thread,
    time::{Duration, Instant},
};

use log::{error, info};
use reqwest::{Client, Response};
use task_kube::{models::Task, task_state::TaskState};

#[tokio::main]
async fn main() {
    env_logger::init();
    let address = match env::var("API_ADDRESS") {
        Ok(val) => val,
        Err(_) => panic!("API_ADDRESS not found in environment"),
    };
    let port = match env::var("API_PORT") {
        Ok(val) => val,
        Err(_) => panic!("API_PORT not found in environment"),
    };
    let load_request_address = format!("http://{}:{}/load-request", address, port);
    let task_request_address = format!("http://{}:{}/task", address, port);

    let client = Client::new();

    loop {
        info!("Sending request to {}", load_request_address);
        let task = request_payload(&client, &load_request_address).await;
        match task {
            Ok(mut task) => {
                task.state = TaskState::InProgress.to_string();
                let result = update_task(&client, &task_request_address, &task).await;
                match result {
                    Ok(_) => {
                        info!("Starting execution of task {}", task.id);
                        let out = execute(&task).await;
                        info!("Task {} executed", task.id);
                        task.state = TaskState::Completed.to_string();
                        let result = update_task(&client, &task_request_address, &task).await;
                        match result {
                            Ok(_) => {
                                info!("{} executed successfully with out {:.2} ", task.id, out)
                            }
                            Err(_) => error!("{} Task failed to update", task.id),
                        }
                    }
                    Err(_) => {
                        error!("{} Task failed to update", task.id)
                    }
                }
            }
            Err(e) => {
                info!("No more payloads, exiting {e}");
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

async fn request_payload(client: &Client, target: &str) -> Result<Task, reqwest::Error> {
    info!("Requesting payload");
    let resp = client.get(target).send().await;
    info!("{:?}", resp);
    match resp {
        Ok(response) => response.json::<Task>().await,
        Err(err) => Err(err),
    }
}

async fn update_task(
    client: &Client,
    target: &str,
    task: &Task,
) -> Result<Response, reqwest::Error> {
    info!("Updating task {} to {}", task.id, task.state);
    client.put(target).json(task).send().await
}

// Create dummy task with high CPU usage
// for the autoscaler to recognise
async fn execute(task: &Task) -> f64 {
    let start = Instant::now();
    let mut x = 0.0_f64;

    // Loop until the duration has passed
    while start.elapsed() < Duration::from_secs(task.delay as u64) {
        // Perform some floating-point operations to keep CPU busy
        x = (x + 1.0).sin().cos().tan();
        if x.is_nan() {
            x = 0.0; // Reset if NaN to avoid stopping the loop
        }
    }
    x
}
