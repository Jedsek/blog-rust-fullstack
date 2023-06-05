use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GithubUserInfo {
    pub id: u32,
    pub login: String,
    pub avatar_url: String,
}
