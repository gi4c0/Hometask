use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, AsRefStr, EnumString, PartialEq, Eq)]
pub enum ProfileKind {
    #[strum(serialize = "client")]
    Client,
    #[strum(serialize = "contractor")]
    Contractor,
}

impl ProfileKind {
    pub fn get_profile_filter(&self) -> &'static str {
        match self {
            ProfileKind::Client => "ClientId",
            ProfileKind::Contractor => "ContractorId",
        }
    }
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
