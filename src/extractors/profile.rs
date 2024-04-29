use std::str::FromStr;

use anyhow::Context;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};

use crate::{
    application::AppCtx, enums::ProfileKind, models::profile::Profile, types::ProfileId,
    utils::err::Error,
};

#[async_trait]
impl<S> FromRequestParts<S> for Profile
where
    AppCtx: FromRef<S>,
    S: Sync + Send,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let profile_id = ProfileId::from_request_parts(parts, state).await?;

        let state = parts
            .extract_with_state::<AppCtx, _>(state)
            .await
            .context("Failed to extract state")?;

        let profile = sqlx::query!(
            r#"
                SELECT
                    id AS "id!",
                    "firstName" AS first_name,
                    "lastName" AS last_name,
                    profession,
                    balance as "balance: Option<f64>",
                    type AS "kind!"
                FROM
                    Profiles
                WHERE
                    id = $1
            "#,
            profile_id.0
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or(Error::Unauthorized)?;

        Ok(Profile {
            id: profile.id.into(),
            first_name: profile.first_name,
            last_name: profile.last_name,
            kind: ProfileKind::from_str(&profile.kind)
                .with_context(|| format!("Invalid profile kind: {}", profile.kind))?,
            balance: profile.balance.flatten(),
            profession: profile.profession,
        })
    }
}
