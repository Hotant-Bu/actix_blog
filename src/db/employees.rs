use sqlx::MySqlPool;
use crate::models::{Employee, EmployeeWithDetails, CreateEmployeeRequest, UpdateEmployeeRequest};

pub async fn create_employee(pool: &MySqlPool, req: &CreateEmployeeRequest) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO employees (user_id, department_id, employee_number, full_name, position, phone, hire_date) VALUES (?, ?, ?, ?, ?, ?, ?)",
        req.user_id,
        req.department_id,
        req.employee_number,
        req.full_name,
        req.position,
        req.phone,
        req.hire_date
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_employee_by_id(pool: &MySqlPool, id: i64) -> Result<Option<Employee>, sqlx::Error> {
    sqlx::query_as::<_, Employee>(
        "SELECT id, user_id, department_id, employee_number, full_name, position, phone, hire_date, created_at, updated_at FROM employees WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_employee_with_details(pool: &MySqlPool, id: i64) -> Result<Option<EmployeeWithDetails>, sqlx::Error> {
    // 先查询员工基本信息
    let employee = match get_employee_by_id(pool, id).await? {
        Some(emp) => emp,
        None => return Ok(None),
    };

    // 查询关联的用户信息
    let user = crate::db::get_user_by_id(pool, employee.user_id).await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    // 查询关联的部门信息（如果有）
    let department_name = if let Some(dept_id) = employee.department_id {
        crate::db::get_department_by_id(pool, dept_id).await?
            .map(|d| d.name)
    } else {
        None
    };

    Ok(Some(EmployeeWithDetails {
        id: employee.id,
        user_id: employee.user_id,
        username: user.username,
        email: user.email,
        department_id: employee.department_id,
        department_name,
        employee_number: employee.employee_number,
        full_name: employee.full_name,
        position: employee.position,
        phone: employee.phone,
        hire_date: employee.hire_date,
        created_at: employee.created_at,
        updated_at: employee.updated_at,
    }))
}

pub async fn get_all_employees_with_details(pool: &MySqlPool) -> Result<Vec<EmployeeWithDetails>, sqlx::Error> {
    // 查询所有员工
    let employees = sqlx::query_as::<_, Employee>(
        "SELECT id, user_id, department_id, employee_number, full_name, position, phone, hire_date, created_at, updated_at FROM employees ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    if employees.is_empty() {
        return Ok(Vec::new());
    }

    // 收集所有 user_id 和 department_id
    let user_ids: Vec<i64> = employees.iter().map(|e| e.user_id).collect();
    let dept_ids: Vec<i64> = employees.iter()
        .filter_map(|e| e.department_id)
        .collect();

    // 批量查询用户信息
    let users = if !user_ids.is_empty() {
        let placeholders = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users WHERE id IN ({})",
            placeholders
        );
        let mut query = sqlx::query_as::<_, crate::models::User>(&query_str);
        for id in &user_ids {
            query = query.bind(id);
        }
        query.fetch_all(pool).await?
    } else {
        Vec::new()
    };

    // 批量查询部门信息
    let departments = if !dept_ids.is_empty() {
        let placeholders = dept_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, name, parent_id, description, created_at, updated_at FROM departments WHERE id IN ({})",
            placeholders
        );
        let mut query = sqlx::query_as::<_, crate::models::Department>(&query_str);
        for id in &dept_ids {
            query = query.bind(id);
        }
        query.fetch_all(pool).await?
    } else {
        Vec::new()
    };

    // 构建映射表
    let user_map: std::collections::HashMap<i64, &crate::models::User> = 
        users.iter().map(|u| (u.id, u)).collect();
    let dept_map: std::collections::HashMap<i64, &crate::models::Department> = 
        departments.iter().map(|d| (d.id, d)).collect();

    // 组装结果
    let mut results = Vec::new();
    for employee in employees {
        if let Some(user) = user_map.get(&employee.user_id) {
            let department_name = employee.department_id
                .and_then(|dept_id| dept_map.get(&dept_id))
                .map(|d| d.name.clone());

            results.push(EmployeeWithDetails {
                id: employee.id,
                user_id: employee.user_id,
                username: user.username.clone(),
                email: user.email.clone(),
                department_id: employee.department_id,
                department_name,
                employee_number: employee.employee_number,
                full_name: employee.full_name,
                position: employee.position,
                phone: employee.phone,
                hire_date: employee.hire_date,
                created_at: employee.created_at,
                updated_at: employee.updated_at,
            });
        }
    }

    Ok(results)
}

pub async fn update_employee(pool: &MySqlPool, id: i64, req: &UpdateEmployeeRequest) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE employees SET ");
    let mut updates = Vec::new();

    if req.department_id.is_some() {
        updates.push("department_id = ?");
    }
    if req.employee_number.is_some() {
        updates.push("employee_number = ?");
    }
    if req.full_name.is_some() {
        updates.push("full_name = ?");
    }
    if req.position.is_some() {
        updates.push("position = ?");
    }
    if req.phone.is_some() {
        updates.push("phone = ?");
    }
    if req.hire_date.is_some() {
        updates.push("hire_date = ?");
    }

    if updates.is_empty() {
        return Ok(false);
    }

    query.push_str(&updates.join(", "));
    query.push_str(", updated_at = NOW() WHERE id = ?");

    let mut sql_query = sqlx::query(&query);
    
    if let Some(department_id) = req.department_id {
        sql_query = sql_query.bind(department_id);
    }
    if let Some(employee_number) = &req.employee_number {
        sql_query = sql_query.bind(employee_number);
    }
    if let Some(full_name) = &req.full_name {
        sql_query = sql_query.bind(full_name);
    }
    if let Some(position) = &req.position {
        sql_query = sql_query.bind(position);
    }
    if let Some(phone) = &req.phone {
        sql_query = sql_query.bind(phone);
    }
    if let Some(hire_date) = req.hire_date {
        sql_query = sql_query.bind(hire_date);
    }
    sql_query = sql_query.bind(id);

    let result = sql_query.execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_employee(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM employees WHERE id = ?", id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}
