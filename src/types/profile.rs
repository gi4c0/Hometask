use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProfileId(pub i64);

impl From<i64> for ProfileId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
