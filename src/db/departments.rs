use sqlx::MySqlPool;
use crate::models::{Department, CreateDepartmentRequest, UpdateDepartmentRequest};

pub async fn create_department(pool: &MySqlPool, req: &CreateDepartmentRequest) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO departments (name, parent_id, description) VALUES (?, ?, ?)",
        req.name,
        req.parent_id,
        req.description
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_department_by_id(pool: &MySqlPool, id: i64) -> Result<Option<Department>, sqlx::Error> {
    sqlx::query_as::<_, Department>(
        "SELECT id, name, parent_id, description, created_at, updated_at FROM departments WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_all_departments(pool: &MySqlPool) -> Result<Vec<Department>, sqlx::Error> {
    sqlx::query_as::<_, Department>(
        "SELECT id, name, parent_id, description, created_at, updated_at FROM departments ORDER BY name"
    )
    .fetch_all(pool)
    .await
}

pub async fn update_department(pool: &MySqlPool, id: i64, req: &UpdateDepartmentRequest) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE departments SET ");
    let mut updates = Vec::new();

    if req.name.is_some() {
        updates.push("name = ?");
    }
    if req.parent_id.is_some() {
        updates.push("parent_id = ?");
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
    if let Some(parent_id) = req.parent_id {
        sql_query = sql_query.bind(parent_id);
    }
    if let Some(description) = &req.description {
        sql_query = sql_query.bind(description);
    }
    sql_query = sql_query.bind(id);

    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_department(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    // 级联处理：将关联员工的部门ID设为NULL
    sqlx::query("UPDATE employees SET department_id = NULL WHERE department_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 将子部门的parent_id设为NULL
    sqlx::query("UPDATE departments SET parent_id = NULL WHERE parent_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // 删除部门本身
    let result = sqlx::query("DELETE FROM departments WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}
