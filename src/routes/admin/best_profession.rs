use anyhow::Context;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::{
    application::AppCtx,
    models::TimeRange,
    utils::response::{AppResult, DataResponse, HandlerDataResponse},
};

pub async fn best_profession(
    time_range: TimeRange,
    state: State<AppCtx>,
) -> HandlerDataResponse<Option<String>> {
    let profession = get_best_profession(&state.db, &time_range).await?;
    Ok(DataResponse::new(profession))
}

async fn get_best_profession(db: &SqlitePool, time_range: &TimeRange) -> AppResult<Option<String>> {
    let result = sqlx::query!(
        r#"
            SELECT
                SUM(price) as sum,
                p.profession
            FROM Jobs j
            JOIN Contracts c ON c.id = j.ContractId
            JOIN Profiles p ON p.id = c.ContractorId
            WHERE j.paymentDate >= $1 AND j.paymentDate <= $2
            GROUP BY p.profession
            ORDER BY sum DESC
            LIMIT 1
        "#,
        time_range.start,
        time_range.end
    )
    .fetch_optional(db)
    .await
    .with_context(|| format!("Failed to fetch best profession"))?;

    Ok(result.map(|row| row.profession))
}
