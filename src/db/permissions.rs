use sqlx::MySqlPool;
use crate::models::{Permission, CreatePermissionRequest, UpdatePermissionRequest};

pub async fn create_permission(pool: &MySqlPool, req: &CreatePermissionRequest) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO permissions (name, resource, action, description) VALUES (?, ?, ?, ?)",
        req.name,
        req.resource,
        req.action,
        req.description
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_permission_by_id(pool: &MySqlPool, id: i64) -> Result<Option<Permission>, sqlx::Error> {
    sqlx::query_as::<_, Permission>(
        "SELECT id, name, resource, action, description, created_at, updated_at FROM permissions WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_all_permissions(pool: &MySqlPool) -> Result<Vec<Permission>, sqlx::Error> {
    sqlx::query_as::<_, Permission>(
        "SELECT id, name, resource, action, description, created_at, updated_at FROM permissions ORDER BY resource, action"
    )
    .fetch_all(pool)
    .await
}

pub async fn update_permission(pool: &MySqlPool, id: i64, req: &UpdatePermissionRequest) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE permissions SET ");
    let mut updates = Vec::new();

    if req.name.is_some() {
        updates.push("name = ?");
    }
    if req.resource.is_some() {
        updates.push("resource = ?");
    }
    if req.action.is_some() {
        updates.push("action = ?");
    }
    if req.description.is_some() {
        updates.push("description = ?");
    }

    if updates.is_empty() {
        return Ok(false);
    }

    query.push_str(&updates.join(", "));
    query.push_str(", updated_at = NOW() WHERE id = ?");

    let mut sql_query = sqlx::query(&query);
    
    if let Some(name) = &req.name {
        sql_query = sql_query.bind(name);
    }
    if let Some(resource) = &req.resource {
        sql_query = sql_query.bind(resource);
    }
    if let Some(action) = &req.action {
        sql_query = sql_query.bind(action);
    }
    if let Some(description) = &req.description {
        sql_query = sql_query.bind(description);
    }
    sql_query = sql_query.bind(id);

    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_permission(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    // 级联删除：先删除角色权限关联
    sqlx::query("DELETE FROM role_permissions WHERE permission_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 删除权限本身
    let result = sqlx::query("DELETE FROM permissions WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}
