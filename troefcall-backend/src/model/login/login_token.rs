use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct LoginToken {
    pub user_id: usize,
    pub exp: u64,
}