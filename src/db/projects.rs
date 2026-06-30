use sqlx::MySqlPool;
use crate::models::{Project, ProjectImage, CreateProjectRequest, UpdateProjectRequest, AddProjectImageRequest};

pub async fn create_project(pool: &MySqlPool, req: &CreateProjectRequest, created_by: i64) -> Result<i64, sqlx::Error> {
    let status = req.status.as_deref().unwrap_or("draft");
    let display_order = req.display_order.unwrap_or(0);
    
    let result = sqlx::query!(
        "INSERT INTO projects (title, description,technologies, cover_image, video_url, status, display_order, created_by) VALUES (?, ?, ?,?, ?, ?, ?, ?)",
        req.title,
        req.description,
        req.technologies,
        req.cover_image,
        req.video_url,
        status,
        display_order,
        created_by
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

/// 根据项目id获取项目信息
pub async fn get_project_by_id(pool: &MySqlPool, id: i64) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "SELECT id, title, description, technologies, cover_image, video_url, status, display_order, created_by, created_at, updated_at FROM projects WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_all_projects(pool: &MySqlPool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "SELECT id, title, description, technologies, cover_image, video_url, status, display_order, created_by, created_at, updated_at FROM projects ORDER BY display_order DESC, created_at DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_published_projects(pool: &MySqlPool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "SELECT id, title, description, technologies, cover_image, video_url, status, display_order, created_by, created_at, updated_at FROM projects WHERE status = 'published' ORDER BY display_order DESC, created_at DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn update_project(pool: &MySqlPool, id: i64, req: &UpdateProjectRequest) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE projects SET ");
    let mut updates = Vec::new();

    if req.id != 0 {
        updates.push("id = ?");
    }
    if req.title.is_some() {
        updates.push("title = ?");
    }
    if req.description.is_some() {
        updates.push("description = ?");
    }
    if req.technologies.is_some() {
        updates.push("technologies= ?");
    }
    if req.cover_image.is_some() {
        updates.push("cover_image = ?");
    }
    if req.video_url.is_some() {
        updates.push("video_url = ?");
    }
    if req.status.is_some() {
        updates.push("status = ?");
    }
    if req.display_order.is_some() {
        updates.push("display_order = ?");
    }

    if updates.is_empty() {
        return Ok(false);
    }

    query.push_str(&updates.join(", "));
    query.push_str(", updated_at = NOW() WHERE id = ?");

    let mut sql_query = sqlx::query(&query);

    if req.id != 0{
        sql_query = sql_query.bind(id);
    }
    if let Some(title) = &req.title {
        sql_query = sql_query.bind(title);
    }
    if let Some(description) = &req.description {
        sql_query = sql_query.bind(description);
    }
    if let Some(technologies) = &req.technologies{
        sql_query = sql_query.bind(technologies);
    }
    if let Some(cover_image) = &req.cover_image {
        sql_query = sql_query.bind(cover_image);
    }
    if let Some(video_url) = &req.video_url {
        sql_query = sql_query.bind(video_url);
    }
    if let Some(status) = &req.status {
        sql_query = sql_query.bind(status);
    }
    if let Some(display_order) = req.display_order {
        sql_query = sql_query.bind(display_order);
    }
    sql_query = sql_query.bind(id);

    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_project(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    // 级联删除：先删除项目图片
    sqlx::query("DELETE FROM project_images WHERE project_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 删除项目本身
    let result = sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn add_project_image(pool: &MySqlPool, req: &AddProjectImageRequest) -> Result<i64, sqlx::Error> {
    let display_order = req.display_order.unwrap_or(0);
    
    let result = sqlx::query!(
        "INSERT INTO project_images (project_id, image_url, caption, display_order) VALUES (?, ?, ?, ?)",
        req.project_id,
        req.image_url,
        req.caption,
        display_order
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_project_images(pool: &MySqlPool, project_id: i64) -> Result<Vec<ProjectImage>, sqlx::Error> {
    sqlx::query_as::<_, ProjectImage>(
        "SELECT id, project_id, image_url, caption, display_order, created_at FROM project_images WHERE project_id = ? ORDER BY display_order, created_at"
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}

pub async fn delete_project_image(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM project_images WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
