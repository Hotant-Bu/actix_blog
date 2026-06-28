use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{CreatePermissionRequest, UpdatePermissionRequest};
use crate::utils::{ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;
use crate::db;

pub async fn create_permission(
    pool: web::Data<MySqlPool>,
    req: web::Json<CreatePermissionRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    match db::create_permission(&pool, &req).await {
        Ok(permission_id) => {
            match db::get_permission_by_id(&pool, permission_id).await {
                Ok(Some(permission)) => ApiResponse::success_with_message("Permission created successfully", permission),
                _ => ErrorResponse::internal_error("Failed to fetch created permission"),
            }
        }
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_all_permissions(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_all_permissions(&pool).await {
        Ok(permissions) => ApiResponse::success(permissions),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_permission(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let permission_id = path.into_inner();
    
    match db::get_permission_by_id(&pool, permission_id).await {
        Ok(Some(permission)) => ApiResponse::success(permission),
        Ok(None) => ErrorResponse::not_found("Permission not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn update_permission(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<UpdatePermissionRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let permission_id = path.into_inner();

    match db::update_permission(&pool, permission_id, &req).await {
        Ok(true) => {
            match db::get_permission_by_id(&pool, permission_id).await {
                Ok(Some(permission)) => ApiResponse::success_with_message("Permission updated successfully", permission),
                _ => ErrorResponse::internal_error("Failed to fetch updated permission"),
            }
        }
        Ok(false) => ErrorResponse::not_found("Permission not found or no changes made"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_permission(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let permission_id = path.into_inner();

    match db::delete_permission(&pool, permission_id).await {
        Ok(true) => ApiResponse::success_with_message("Permission deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("Permission not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/permissions")
            .wrap(AuthMiddleware)
            .route("", web::post().to(create_permission))
            .route("", web::get().to(get_all_permissions))
            .route("/{id}", web::get().to(get_permission))
            .route("/{id}", web::put().to(update_permission))
            .route("/{id}", web::delete().to(delete_permission))
    );
}
