use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{CreateRoleRequest, UpdateRoleRequest, AssignPermissionRequest};
use crate::utils::{ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;
use crate::db;

pub async fn create_role(
    pool: web::Data<MySqlPool>,
    req: web::Json<CreateRoleRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    match db::create_role(&pool, &req).await {
        Ok(role_id) => {
            match db::get_role_by_id(&pool, role_id).await {
                Ok(Some(role)) => ApiResponse::success_with_message("Role created successfully", role),
                _ => ErrorResponse::internal_error("Failed to fetch created role"),
            }
        }
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_all_roles(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_all_roles(&pool).await {
        Ok(roles) => ApiResponse::success(roles),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_role(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let role_id = path.into_inner();
    
    match db::get_role_by_id(&pool, role_id).await {
        Ok(Some(role)) => ApiResponse::success(role),
        Ok(None) => ErrorResponse::not_found("Role not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn update_role(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<UpdateRoleRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let role_id = path.into_inner();

    match db::update_role(&pool, role_id, &req).await {
        Ok(true) => {
            match db::get_role_by_id(&pool, role_id).await {
                Ok(Some(role)) => ApiResponse::success_with_message("Role updated successfully", role),
                _ => ErrorResponse::internal_error("Failed to fetch updated role"),
            }
        }
        Ok(false) => ErrorResponse::not_found("Role not found or no changes made"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_role(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let role_id = path.into_inner();

    match db::delete_role(&pool, role_id).await {
        Ok(true) => ApiResponse::success_with_message("Role deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("Role not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn assign_permissions(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<AssignPermissionRequest>,
) -> HttpResponse {
    let role_id = path.into_inner();

    match db::assign_permissions_to_role(&pool, role_id, &req.permission_ids).await {
        Ok(_) => ApiResponse::success_with_message("Permissions assigned successfully", ()),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_role_permissions(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let role_id = path.into_inner();

    match db::get_role_permissions(&pool, role_id).await {
        Ok(permissions) => ApiResponse::success(permissions),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .wrap(AuthMiddleware)
            .route("", web::post().to(create_role))
            .route("", web::get().to(get_all_roles))
            .route("/{id}", web::get().to(get_role))
            .route("/{id}", web::put().to(update_role))
            .route("/{id}", web::delete().to(delete_role))
            .route("/{id}/permissions", web::post().to(assign_permissions))
            .route("/{id}/permissions", web::get().to(get_role_permissions))
    );
}
