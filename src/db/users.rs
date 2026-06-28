use sqlx::MySqlPool;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};

pub async fn create_user(pool: &MySqlPool, req: &CreateUserRequest, password_hash: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
        req.username,
        req.email,
        password_hash
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_user_by_id(pool: &MySqlPool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn update_user(pool: &MySqlPool, id: i64, req: &UpdateUserRequest) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE users SET ");
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(username) = &req.username {
        updates.push("username = ?");
        params.push(username.clone());
    }
    if let Some(email) = &req.email {
        updates.push("email = ?");
        params.push(email.clone());
    }
    if let Some(is_active) = req.is_active {
        updates.push("is_active = ?");
        params.push(is_active.to_string());
    }

    if updates.is_empty() {
        return Ok(false);
    }

    query.push_str(&updates.join(", "));
    query.push_str(", updated_at = NOW() WHERE id = ?");
    params.push(id.to_string());

    let mut sql_query = sqlx::query(&query);
    for param in &params[..params.len()-1] {
        sql_query = sql_query.bind(param);
    }
    sql_query = sql_query.bind(id);

    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_user(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    // 级联删除：先删除关联数据
    // 1. 删除用户角色关联
    sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 2. 删除员工记录
    sqlx::query("DELETE FROM employees WHERE user_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 3. 删除用户创建的项目
    let project_ids: Vec<i64> = sqlx::query_scalar(
        "SELECT id FROM projects WHERE created_by = ?"
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    
    // 删除项目的图片
    for project_id in &project_ids {
        sqlx::query("DELETE FROM project_images WHERE project_id = ?")
            .bind(project_id)
            .execute(pool)
            .await?;
    }
    
    // 删除项目本身
    sqlx::query("DELETE FROM projects WHERE created_by = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 4. 最后删除用户
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn get_user_permissions(pool: &MySqlPool, user_id: i64) -> Result<Vec<String>, sqlx::Error> {
    // 1. 查询用户的角色ID列表
    let user_role_ids: Vec<i64> = sqlx::query_scalar(
        "SELECT role_id FROM user_roles WHERE user_id = ?"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    if user_role_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 2. 查询这些角色关联的权限ID列表
    let placeholders = user_role_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!(
        "SELECT DISTINCT permission_id FROM role_permissions WHERE role_id IN ({})",
        placeholders
    );
    let mut query = sqlx::query_scalar::<_, i64>(&query_str);
    for role_id in &user_role_ids {
        query = query.bind(role_id);
    }
    let permission_ids: Vec<i64> = query.fetch_all(pool).await?;

    if permission_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 3. 查询权限名称
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

pub async fn assign_roles_to_user(pool: &MySqlPool, user_id: i64, role_ids: &[i64]) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM user_roles WHERE user_id = ?", user_id)
        .execute(pool)
        .await?;

    for role_id in role_ids {
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES (?, ?)",
            user_id,
            role_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
