use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRole {
    pub user_id: i64,
    pub role_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role_ids: Vec<i64>,
}
