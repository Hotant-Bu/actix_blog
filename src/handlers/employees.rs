use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{CreateEmployeeRequest, UpdateEmployeeRequest};
use crate::utils::{ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;
use crate::db;

pub async fn create_employee(
    pool: web::Data<MySqlPool>,
    req: web::Json<CreateEmployeeRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    match db::create_employee(&pool, &req).await {
        Ok(employee_id) => {
            match db::get_employee_with_details(&pool, employee_id).await {
                Ok(Some(employee)) => ApiResponse::success_with_message("Employee created successfully", employee),
                _ => ErrorResponse::internal_error("Failed to fetch created employee"),
            }
        }
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_all_employees(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_all_employees_with_details(&pool).await {
        Ok(employees) => ApiResponse::success(employees),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_employee(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let employee_id = path.into_inner();
    
    match db::get_employee_with_details(&pool, employee_id).await {
        Ok(Some(employee)) => ApiResponse::success(employee),
        Ok(None) => ErrorResponse::not_found("Employee not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn update_employee(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<UpdateEmployeeRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let employee_id = path.into_inner();

    match db::update_employee(&pool, employee_id, &req).await {
        Ok(true) => {
            match db::get_employee_with_details(&pool, employee_id).await {
                Ok(Some(employee)) => ApiResponse::success_with_message("Employee updated successfully", employee),
                _ => ErrorResponse::internal_error("Failed to fetch updated employee"),
            }
        }
        Ok(false) => ErrorResponse::not_found("Employee not found or no changes made"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_employee(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let employee_id = path.into_inner();

    match db::delete_employee(&pool, employee_id).await {
        Ok(true) => ApiResponse::success_with_message("Employee deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("Employee not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/employees")
            .wrap(AuthMiddleware)
            .route("", web::post().to(create_employee))
            .route("", web::get().to(get_all_employees))
            .route("/{id}", web::get().to(get_employee))
            .route("/{id}", web::put().to(update_employee))
            .route("/{id}", web::delete().to(delete_employee))
    );
}
