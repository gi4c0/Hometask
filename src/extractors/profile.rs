use std::str::FromStr;

use anyhow::Context;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, HeaderMap},
    RequestPartsExt,
};

use crate::{application::AppCtx, enums::ProfileType, models::profile::Profile, utils::err::Error};

#[async_trait]
impl<S> FromRequestParts<S> for Profile
where
    AppCtx: FromRef<S>,
    S: Sync + Send,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|e| Error::Internal(e.into()))?;

        let profile_id = headers.get("profile_id").ok_or(Error::Unauthorized)?;

        let profile_id = profile_id
            .to_str()
            .map_err(|_| Error::Unauthorized)?
            .parse::<i64>()
            .map_err(|_| Error::Unauthorized)?;

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
            profile_id
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or(Error::Unauthorized)?;

        Ok(Profile {
            id: profile.id.into(),
            first_name: profile.first_name,
            last_name: profile.last_name,
            kind: ProfileType::from_str(&profile.kind)
                .with_context(|| format!("Invalid profile kind: {}", profile.kind))?,
            balance: profile.balance.flatten(),
            profession: profile.profession,
        })
    }
}
