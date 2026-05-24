use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}
