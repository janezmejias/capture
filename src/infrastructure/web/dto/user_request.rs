use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub id: u32,
}