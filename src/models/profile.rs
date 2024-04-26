use serde::{Deserialize, Serialize};

use crate::{enums::ProfileKind, types::ProfileId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: ProfileId,
    pub first_name: String,
    pub last_name: String,
    pub profession: String,
    pub balance: Option<f64>,
    pub kind: ProfileKind,
}
