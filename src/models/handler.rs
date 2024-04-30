use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct Total {
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(with = "time::serde::iso8601")]
    pub start: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end: OffsetDateTime,
}
