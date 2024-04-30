use anyhow::Context;
use sqlx::{Sqlite, Transaction};

use crate::{types::ProfileId, utils::response::AppResult};

pub async fn transfer_money(
    t: &mut Transaction<'_, Sqlite>,
    delta: f64,
    source_profile_id: ProfileId,
    target_profile_id: ProfileId,
) -> AppResult<()> {
    update_balance(t, -delta, source_profile_id).await?;
    update_balance(t, delta, target_profile_id).await?;

    Ok(())
}

async fn update_balance(
    t: &mut Transaction<'_, Sqlite>,
    delta: f64,
    profile_id: ProfileId,
) -> AppResult<()> {
    sqlx::query!(
        r#"
            UPDATE Profiles SET balance = balance + $1
            WHERE id = $2
        "#,
        delta,
        profile_id.0
    )
    .execute(&mut **t)
    .await
    .with_context(|| {
        format!(
            "Failed to update balance for profile: {} (delta: {})",
            profile_id, delta
        )
    })?;

    Ok(())
}
