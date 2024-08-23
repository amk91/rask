use serde::{Deserialize, Serialize};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime,
    Utc
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
    New,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub uuid: Uuid,
    pub title: String,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub complete_by: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub percentage: Option<u8>,
}

impl Task {
    pub fn new(
        uuid_node: &[u8; 6],
        title: String,
        created: DateTime<Utc>,
        complete_by: Option<DateTime<Utc>>,
        description: Option<String>,
        status: TaskStatus,
        percentage: Option<u8>,
    ) -> Task {
        Task {
            uuid: Uuid::now_v6(uuid_node),
            title,
            created,
            complete_by,
            description,
            status,
            percentage
        }
    }
}
