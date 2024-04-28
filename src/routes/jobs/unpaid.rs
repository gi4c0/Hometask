use anyhow::Context;
use axum::extract::{Query, State};
use sqlx::SqlitePool;

use crate::{
    application::AppCtx,
    enums::ContractStatus,
    models::{profile::Profile, Criteria, Total},
    utils::response::{AppResult, HandlerPaginatedResponse, PaginatedResponse},
};

use super::{Job, RawJob};

pub async fn get_unpaid_jobs(
    state: State<AppCtx>,
    profile: Profile,
    Query(criteria): Query<Criteria>,
) -> HandlerPaginatedResponse<Job> {
    let (total, jobs) = tokio::try_join!(
        get_total_unpaid_jobs(&state.db, &profile),
        load_unpaid_jobs(&state.db, &profile, &criteria)
    )?;

    Ok(PaginatedResponse::new(jobs, total))
}

async fn load_unpaid_jobs(
    db: &SqlitePool,
    profile: &Profile,
    criteria: &Criteria,
) -> AppResult<Vec<Job>> {
    let new_status = ContractStatus::New.as_ref();
    let in_progress_status = ContractStatus::InProgress.as_ref();

    let raw_jobs: Vec<RawJob> = sqlx::query_as(&format!(
        r#"
            SELECT
                j.id,
                j.description,
                j.price,
                j.paid,
                j.paymentDate as payment_date,
                j.createdAt as created_at,
                j.updatedAt as updated_at,
                j.ContractId as contract_id
            FROM Jobs j
            JOIN Contracts c on c.id = j.ContractId
            WHERE
                c.{} = $1
            AND c.status IN ($2, $3)
            AND j.paid IS NULL
            LIMIT $4 OFFSET $5
        "#,
        profile.kind.get_profile_filter(),
    ))
    .bind(profile.id.0)
    .bind(new_status)
    .bind(in_progress_status)
    .bind(criteria.limit.unwrap_or(50) as i64)
    .bind(criteria.offset.unwrap_or(0) as i64)
    .fetch_all(db)
    .await
    .with_context(|| format!("Failed to fetch unpaid jobs for profile: {}", profile.id))?;

    let mut result = vec![];

    for row in raw_jobs {
        result.push(row.into_job());
    }

    Ok(result)
}

async fn get_total_unpaid_jobs(db: &SqlitePool, profile: &Profile) -> AppResult<i32> {
    let new_status = ContractStatus::New.as_ref();
    let in_progress_status = ContractStatus::InProgress.as_ref();

    let result: Total = sqlx::query_as(&format!(
        r#"
            SELECT
                COUNT(*) as total
            FROM Jobs j
            JOIN Contracts c on c.id = j.ContractId
            WHERE
                c.{} = $1
            AND j.paid IS NULL
            AND c.status IN ($2, $3)
        "#,
        profile.kind.get_profile_filter(),
    ))
    .bind(profile.id.0)
    .bind(new_status)
    .bind(in_progress_status)
    .fetch_one(db)
    .await
    .with_context(|| {
        format!(
            "Failed to fetch total unpaid jobs for profile: {}",
            profile.id
        )
    })?;

    Ok(result.total)
}
