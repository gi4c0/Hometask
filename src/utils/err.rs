use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use thiserror::Error;
use tracing::error;
use validator::{ValidationErrors, ValidationErrorsKind};

use super::response::ErrorResponse;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error("Database error")]
    Db(#[from] sqlx::Error),

    #[error("Malformed JSON")]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("Failed to parse query params")]
    AxumQueryRejection(#[from] QueryRejection),

    #[error("Not found")]
    NotFound,

    #[error("{0}")]
    BadRequest(String),

    #[error("Failed to parse path params")]
    AxumPathRejection(#[from] PathRejection),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,
}

const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

impl Error {
    pub fn get_status(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Db(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AxumJsonRejection(_)
            | Self::BadRequest(_)
            | Self::AxumQueryRejection(_)
            | Self::ValidationError(_)
            | Self::AxumPathRejection(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Db(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::message(INTERNAL_SERVER_ERROR)),
                )
                    .into_response()
            }

            Self::Internal(e) => {
                error!("Internal server error: {}", format_error(e));
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::message(INTERNAL_SERVER_ERROR)),
                )
                    .into_response();
            }

            Self::ValidationError(e) => {
                let json =
                    ErrorResponse::with_data("Input validation error", format_validator_errors(&e));
                return (StatusCode::BAD_REQUEST, Json(json)).into_response();
            }
            _ => (),
        };

        let message = self.to_string();
        error!(message);

        (self.get_status(), Json(ErrorResponse::message(message))).into_response()
    }
}

fn format_error(err: anyhow::Error) -> String {
    let mut stack = vec![];

    for cause in err.chain() {
        stack.push(cause.to_string());
    }

    stack.join("\n")
}

fn format_validator_errors(validation_errs: &ValidationErrors) -> Vec<String> {
    validation_errs
        .errors()
        .iter()
        .map(|(field, error_kind)| match error_kind {
            ValidationErrorsKind::Field(errors) => {
                let errors: String = errors
                    .iter()
                    .map(|e| e.message.as_ref().unwrap_or(&e.code).to_string())
                    .collect::<Vec<String>>()
                    .join("; ");

                format!("{field}: {}", errors)
            }

            ValidationErrorsKind::Struct(validated_struct) => {
                let errors: String = format_validator_errors(validated_struct).join("; ");
                format!("{field}: {}", errors)
            }

            ValidationErrorsKind::List(list) => {
                let errors: String = list
                    .values()
                    .map(|e| format_validator_errors(e).join("; "))
                    .collect::<Vec<String>>()
                    .join("; ");

                format!("{field}: {}", errors)
            }
        })
        .collect()
}
