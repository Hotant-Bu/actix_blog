use actix_web::{web, HttpResponse, HttpRequest,HttpMessage};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{UpdateUserRequest, UserResponse, AssignRoleRequest};
use crate::utils::{Claims, ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;
use crate::db;

pub async fn get_current_user(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();
    
    if let Some(claims) = claims {
        match db::get_user_by_id(&pool, claims.sub).await {
            Ok(Some(user)) => {
                let response: UserResponse = user.into();
                ApiResponse::success(response)
            }
            Ok(None) => ErrorResponse::not_found("User not found"),
            Err(_) => ErrorResponse::internal_error("Database error"),
        }
    } else {
        ErrorResponse::unauthorized("Unauthorized")
    }
}

pub async fn get_all_users(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_all_users(&pool).await {
        Ok(users) => {
            let response: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
            ApiResponse::success(response)
        }
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_user(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let user_id = path.into_inner();
    
    match db::get_user_by_id(&pool, user_id).await {
        Ok(Some(user)) => {
            let response: UserResponse = user.into();
            ApiResponse::success(response)
        }
        Ok(None) => ErrorResponse::not_found("User not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn update_user(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let user_id = path.into_inner();

    match db::update_user(&pool, user_id, &req).await {
        Ok(true) => {
            match db::get_user_by_id(&pool, user_id).await {
                Ok(Some(user)) => {
                    let response: UserResponse = user.into();
                    ApiResponse::success_with_message("User updated successfully", response)
                }
                _ => ErrorResponse::internal_error("Failed to fetch updated user"),
            }
        }
        Ok(false) => ErrorResponse::not_found("User not found or no changes made"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_user(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match db::delete_user(&pool, user_id).await {
        Ok(true) => ApiResponse::success_with_message("User deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("User not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn assign_roles(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<AssignRoleRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match db::assign_roles_to_user(&pool, user_id, &req.role_ids).await {
        Ok(_) => ApiResponse::success_with_message("Roles assigned successfully", ()),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_user_permissions(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match db::get_user_permissions(&pool, user_id).await {
        Ok(permissions) => ApiResponse::success(permissions),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(AuthMiddleware)
            .route("/me", web::get().to(get_current_user))
            .route("", web::get().to(get_all_users))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            .route("/{id}/roles", web::post().to(assign_roles))
            .route("/{id}/permissions", web::get().to(get_user_permissions))
    );
}
