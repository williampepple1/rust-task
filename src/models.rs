// models.rs

use diesel::prelude::*;
use diesel::table;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
