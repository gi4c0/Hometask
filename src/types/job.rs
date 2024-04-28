use derive_more::From;
use std::{borrow::Cow, collections::HashMap};

use parse_display::Display;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Display, From)]
pub struct JobId(pub i64);

pub fn validate_job_id(value: &JobId) -> Result<(), ValidationError> {
    if value.0 < 0 {
        return Err(ValidationError {
            code: Cow::from("invalid_job_id"),
            params: HashMap::new(),
            message: Some(Cow::from("job id cannot be less then zero")),
        });
    }

    Ok(())
}
