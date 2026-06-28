use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{CreateDepartmentRequest, UpdateDepartmentRequest};
use crate::utils::{ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;
use crate::db;

pub async fn create_department(
    pool: web::Data<MySqlPool>,
    req: web::Json<CreateDepartmentRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    match db::create_department(&pool, &req).await {
        Ok(department_id) => {
            match db::get_department_by_id(&pool, department_id).await {
                Ok(Some(department)) => ApiResponse::success_with_message("Department created successfully", department),
                _ => ErrorResponse::internal_error("Failed to fetch created department"),
            }
        }
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_all_departments(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_all_departments(&pool).await {
        Ok(departments) => ApiResponse::success(departments),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_department(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let department_id = path.into_inner();
    
    match db::get_department_by_id(&pool, department_id).await {
        Ok(Some(department)) => ApiResponse::success(department),
        Ok(None) => ErrorResponse::not_found("Department not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn update_department(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<UpdateDepartmentRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let department_id = path.into_inner();

    match db::update_department(&pool, department_id, &req).await {
        Ok(true) => {
            match db::get_department_by_id(&pool, department_id).await {
                Ok(Some(department)) => ApiResponse::success_with_message("Department updated successfully", department),
                _ => ErrorResponse::internal_error("Failed to fetch updated department"),
            }
        }
        Ok(false) => ErrorResponse::not_found("Department not found or no changes made"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_department(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let department_id = path.into_inner();

    match db::delete_department(&pool, department_id).await {
        Ok(true) => ApiResponse::success_with_message("Department deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("Department not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/departments")
            .wrap(AuthMiddleware)
            .route("", web::post().to(create_department))
            .route("", web::get().to(get_all_departments))
            .route("/{id}", web::get().to(get_department))
            .route("/{id}", web::put().to(update_department))
            .route("/{id}", web::delete().to(delete_department))
    );
}
