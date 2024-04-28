use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

pub mod profile;

#[derive(FromRow)]
pub struct Total {
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Criteria {
    #[validate(range(max = 100))]
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
