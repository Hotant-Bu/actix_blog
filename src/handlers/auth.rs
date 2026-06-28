use actix_web::{web, HttpResponse};
use redis::aio::ConnectionManager;
use serde::Serialize;
use sqlx::MySqlPool;
use validator::Validate;

use crate::config::Config;
use crate::db;
use crate::models::{CreateUserRequest, LoginRequest};
use crate::utils::{
    create_jwt, hash_password, redis_cache, verify_password, ApiResponse, ErrorResponse,
};

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub username: String,
}

pub async fn login(
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    redis: web::Data<ConnectionManager>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    match db::get_user_by_username(&pool, &req.username).await {
        Ok(Some(user)) => {
            match verify_password(&req.password, &user.password_hash) {
                Ok(true) => {
                    if !user.is_active {
                        return ErrorResponse::forbidden("User account is inactive");
                    }

                    match create_jwt(user.id, &user.username, &config.jwt_secret) {
                        Ok(token) => {
                            // 将token存入redis中
                            let expire_time =
                                chrono::Local::now().timestamp() + config.jwt_expiration;
                            if let Err(e) = redis_cache::set_token(
                                &redis,
                                &token,
                                user.id,
                                &user.username,
                                expire_time,
                                config.jwt_expiration,
                            )
                            .await
                            {
                                log::error!("Failed to cache token in Redis: {:?}", e);
                            }

                            let response = LoginResponse {
                                token,
                                user_id: user.id,
                                username: user.username,
                            };
                            ApiResponse::success(response)
                        }
                        Err(_) => ErrorResponse::internal_error("Failed to create token"),
                    }
                }
                Ok(false) => ErrorResponse::unauthorized("Invalid credentials"),
                Err(_) => ErrorResponse::internal_error("Password verification failed"),
            }
        }
        Ok(None) => ErrorResponse::unauthorized("Invalid credentials"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn register(
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    redis: web::Data<ConnectionManager>,
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        log::error!("Validation error: {:?}", errors);
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    if let Ok(Some(_)) = db::get_user_by_username(&pool, &req.username).await {
        return ErrorResponse::bad_request("Username already exists");
    }

    match hash_password(&req.password) {
        Ok(password_hash) => {
            match db::create_user(&pool, &req, &password_hash).await {
                Ok(user_id) => {
                    match create_jwt(user_id, &req.username, &config.jwt_secret) {
                        Ok(token) => {
                            // 构造过期时间
                            let expire_time =
                                chrono::Local::now().timestamp() + config.jwt_expiration;
                            // 可将token存入redis
                            if let Err(e) = redis_cache::set_token(
                                &redis,
                                &token,
                                user_id,
                                &req.username,
                                expire_time,
                                config.jwt_expiration,
                            )
                            .await
                            {
                                log::error!("Failed to cache token in Redis: {:?}", e);
                            }

                            let response = LoginResponse {
                                token,
                                user_id,
                                username: req.username.clone(),
                            };

                            ApiResponse::success_with_message(
                                "User registered successfully",
                                response,
                            )
                        }
                        Err(_) => ErrorResponse::internal_error("Failed to create token"),
                    }
                }
                Err(e) => {
                    log::error!("Failed to create user: {:?}", e);
                    ErrorResponse::internal_error("Failed to create user")
                }
            }
        }
        Err(e) => {
            log::error!("Failed to create user: {:?}", e);
            ErrorResponse::internal_error("Failed to hash password")
        }
    }
}

pub async fn logout(
    redis: web::Data<ConnectionManager>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    use actix_web::HttpResponse;

    // 从请求头获取token
    if let Some(auth_header) = req.headers().get("Authorization") {
        // 将auth_header转为字符串
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                // Redis删除token
                if let Err(e) = redis_cache::delete_token(&redis, token).await {
                    log::error!("Failed to delete token from Redis: {:?}", e);
                    return ErrorResponse::internal_error("Failed to logout");
                }
                return ApiResponse::success_with_message("Logged out successfully",());
            }
        }
    }
    ErrorResponse::bad_request("Invalid Authorization header")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/logout", web::post().to(logout)),
    );
}
