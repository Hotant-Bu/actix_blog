use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RolePermission {
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_ids: Vec<i64>,
}
