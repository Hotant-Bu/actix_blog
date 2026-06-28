use sqlx::MySqlPool;
use crate::models::{Role, CreateRoleRequest, UpdateRoleRequest, RoleWithPermissions};

pub async fn create_role(pool: &MySqlPool, req: &CreateRoleRequest) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO roles (name, description) VALUES (?, ?)",
        req.name,
        req.description
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_role_by_id(pool: &MySqlPool, id: i64) -> Result<Option<Role>, sqlx::Error> {
    sqlx::query_as::<_, Role>(
        "SELECT id, name, description, created_at, updated_at FROM roles WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_all_roles(pool: &MySqlPool) -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as::<_, Role>(
        "SELECT id, name, description, created_at, updated_at FROM roles ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn update_role(pool: &MySqlPool, id: i64, req: &UpdateRoleRequest) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE roles SET ");
    let mut updates = Vec::new();

    if req.name.is_some() {
        updates.push("name = ?");
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
    if let Some(description) = &req.description {
        sql_query = sql_query.bind(description);
    }
    sql_query = sql_query.bind(id);

    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_role(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    // 级联删除：先删除关联数据
    // 1. 删除角色权限关联
    sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 2. 删除用户角色关联
    sqlx::query("DELETE FROM user_roles WHERE role_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 3. 删除角色本身
    let result = sqlx::query("DELETE FROM roles WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn assign_permissions_to_role(pool: &MySqlPool, role_id: i64, permission_ids: &[i64]) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM role_permissions WHERE role_id = ?", role_id)
        .execute(pool)
        .await?;

    for permission_id in permission_ids {
        sqlx::query!(
            "INSERT INTO role_permissions (role_id, permission_id) VALUES (?, ?)",
            role_id,
            permission_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_role_permissions(pool: &MySqlPool, role_id: i64) -> Result<Vec<String>, sqlx::Error> {
    // 1. 查询角色关联的权限ID列表
    let permission_ids: Vec<i64> = sqlx::query_scalar(
        "SELECT permission_id FROM role_permissions WHERE role_id = ?"
    )
    .bind(role_id)
    .fetch_all(pool)
    .await?;

    if permission_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 2. 查询权限名称
    let placeholders = permission_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!(
        "SELECT name FROM permissions WHERE id IN ({})",
        placeholders
    );
    let mut query = sqlx::query_scalar::<_, String>(&query_str);
    for perm_id in &permission_ids {
        query = query.bind(perm_id);
    }
    let permissions = query.fetch_all(pool).await?;

    Ok(permissions)
}
