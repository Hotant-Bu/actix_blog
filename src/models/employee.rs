use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Employee {
    pub id: i64,
    pub user_id: i64,
    pub department_id: Option<i64>,
    pub employee_number: String,
    pub full_name: String,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub hire_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmployeeRequest {
    pub user_id: i64,
    pub department_id: Option<i64>,
    #[validate(length(min = 1, max = 50))]
    pub employee_number: String,
    #[validate(length(min = 2, max = 100))]
    pub full_name: String,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub hire_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmployeeRequest {
    pub department_id: Option<i64>,
    #[validate(length(min = 1, max = 50))]
    pub employee_number: Option<String>,
    #[validate(length(min = 2, max = 100))]
    pub full_name: Option<String>,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub hire_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmployeeWithDetails {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub department_id: Option<i64>,
    pub department_name: Option<String>,
    pub employee_number: String,
    pub full_name: String,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub hire_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
