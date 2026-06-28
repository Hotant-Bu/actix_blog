use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(length(min = 2, max = 50))]
    pub resource: String,
    #[validate(length(min = 2, max = 50))]
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePermissionRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 2, max = 50))]
    pub resource: Option<String>,
    #[validate(length(min = 2, max = 50))]
    pub action: Option<String>,
    pub description: Option<String>,
}
