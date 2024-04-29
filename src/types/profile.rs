use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use derive_more::From;
use parse_display::Display;
use serde::{Deserialize, Serialize};

use crate::utils::err::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, From, Display)]
pub struct ProfileId(pub i64);

#[async_trait]
impl<S> FromRequestParts<S> for ProfileId
where
    S: Send + Sync,
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

        Ok(Self(profile_id))
    }
}
