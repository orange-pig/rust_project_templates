use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}
