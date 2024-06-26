use derive_more::From;
use std::{borrow::Cow, collections::HashMap};

use parse_display::Display;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Display, From)]
pub struct ContractId(pub i64);

pub fn validate_contract_id(value: &ContractId) -> Result<(), ValidationError> {
    if value.0 < 0 {
        return Err(ValidationError {
            code: Cow::from("invalid_contract_id"),
            params: HashMap::new(),
            message: Some(Cow::from("Contract id cannot be less then zero")),
        });
    }

    Ok(())
}
