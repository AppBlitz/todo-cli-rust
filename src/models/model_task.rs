use crate::util::enum_task::StatusTaks;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Todo {
    pub id: usize,
    pub description: String,
    pub status: StatusTaks,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
