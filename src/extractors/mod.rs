use std::{borrow::Cow, collections::HashMap};

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;
use validator::{ValidationError, ValidationErrors};

use crate::utils::err::Error;

pub mod profile;
pub mod validate;

pub struct QueryPagination {
    pub limit: usize,
    pub offset: usize,
}

#[derive(Deserialize)]
struct MaybePagination {
    limit: Option<usize>,
    offset: Option<usize>,
}

#[async_trait]
impl<S> FromRequestParts<S> for QueryPagination
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<MaybePagination>::from_request_parts(parts, state).await?;
        let limit = query.limit.unwrap_or(50);

        if limit > 100 {
            let err = ValidationError {
                code: Cow::from("limits_out_of_bounds"),
                params: HashMap::new(),
                message: Some(Cow::from("Limit is too high. Max allowed: 100")),
            };

            let mut errors = ValidationErrors::new();
            errors.add("limit", err);

            return Err(Error::ValidationError(errors));
        }

        Ok(QueryPagination {
            limit: query.limit.unwrap_or(50),
            offset: query.offset.unwrap_or(0),
        })
    }
}
