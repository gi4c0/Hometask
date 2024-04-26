use std::fmt::Display;

use axum::Json;
use serde::{Deserialize, Serialize};

use super::err::Error;

#[derive(Serialize)]
pub struct ErrorResponse<T> {
    message: String,
    data: Option<T>,
}

impl ErrorResponse<()> {
    pub fn message<Message: Display>(err_message: Message) -> ErrorResponse<()> {
        ErrorResponse {
            message: err_message.to_string(),
            data: None,
        }
    }
}

impl<T> ErrorResponse<T> {
    pub fn with_data<Message: Display>(message: Message, data: T) -> Self {
        ErrorResponse {
            message: message.to_string(),
            data: Some(data),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i32,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i32) -> Json<Self> {
        Json(PaginatedResponse { data, total })
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

impl<T: Serialize> DataResponse<T> {
    pub fn new(data: T) -> Json<Self> {
        Json(DataResponse { data })
    }
}

pub type AppResult<T> = Result<T, Error>;
pub type HandlerDataResponse<T> = AppResult<Json<DataResponse<T>>>;
pub type HandlerPaginatedResponse<T> = AppResult<Json<PaginatedResponse<T>>>;
