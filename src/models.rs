use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::task)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Deserialize, Serialize, AsChangeset)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub delay: i32,
    pub state: String,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::schema::task)]
pub struct NewTask<'a> {
    pub description: &'a str,
    pub delay: i32,
    pub state: &'a str,
}
