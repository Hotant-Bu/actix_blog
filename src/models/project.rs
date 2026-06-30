use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub technologies: Option<String>,
    pub cover_image: Option<String>,
    pub video_url: Option<String>,
    pub status: String,
    pub display_order: i32,
    pub created_by: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 2, max = 200))]
    pub title: String,
    pub description: Option<String>,
    pub technologies: Option<String>,
    pub cover_image: Option<String>,
    pub video_url: Option<String>,
    pub status: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProjectRequest {
    // 添加serde注解，避免在query string过程中因为解析的入参请求体没有id与之对应而报错，
    // 反序列化报错
    #[serde(default)]
    pub id: i64,
    #[validate(length(min = 2, max = 200))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub technologies: Option<String>,
    pub cover_image: Option<String>,
    pub video_url: Option<String>,
    pub status: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProjectImage {
    pub id: i64,
    pub project_id: i64,
    pub image_url: String,
    pub caption: Option<String>,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddProjectImageRequest {
    pub project_id: i64,
    #[validate(length(min = 1))]
    pub image_url: String,
    pub caption: Option<String>,
    pub display_order: Option<i32>,
}
