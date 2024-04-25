use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, AsRefStr, EnumString)]
pub enum ProfileType {
    #[strum(serialize = "client")]
    Client,
    #[strum(serialize = "contractor")]
    Contractor,
}

#[derive(Debug, EnumString, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    #[strum(serialize = "new")]
    New,
    #[strum(serialize = "in_progress")]
    InProgress,
    #[strum(serialize = "terminated")]
    Terminated,
}
