use derive_more::From;
use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, From, Display)]
pub struct ProfileId(pub i64);
