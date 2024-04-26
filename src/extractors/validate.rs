use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Path, Request},
    http::request::Parts,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::err::Error;

pub struct ValidateJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidateJson<T>
where
    S: Sync + Send,
    T: DeserializeOwned + Validate,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;

        Ok(ValidateJson(value))
    }
}

pub struct ValidateParams<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ValidateParams<T>
where
    S: Sync + Send,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request_parts(parts, state).await?;
        value.validate()?;

        Ok(ValidateParams(value))
    }
}
