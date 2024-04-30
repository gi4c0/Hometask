use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{macros::format_description, PrimitiveDateTime};

use crate::utils::err::Error;

#[derive(FromRow)]
pub struct Total {
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: PrimitiveDateTime,
    pub end: PrimitiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawTimeRange {
    start: String,
    end: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for TimeRange
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query: Query<RawTimeRange> = Query::from_request_parts(parts, state).await?;

        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

        Ok(Self {
            start: PrimitiveDateTime::parse(&query.start, &format).map_err(|_| {
                Error::BadRequest(format!(
                    "Invalid time format. Expected yyyy-mm-dd hh::mm::ss +00. Given: {}",
                    query.start
                ))
            })?,
            end: PrimitiveDateTime::parse(&query.end, &format).map_err(|_| {
                Error::BadRequest(format!(
                    "Invalid time format. Expected yyyy-mm-dd hh::mm::ss +00. Given: {}",
                    query.end
                ))
            })?,
        })
    }
}
