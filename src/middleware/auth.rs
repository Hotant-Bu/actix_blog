use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, web, Error, HttpMessage, HttpResponse};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;
use redis::aio::ConnectionManager;
use crate::utils::{redis_cache, verify_jwt, ApiResponse, ErrorResponse};
use crate::config::Config;

/// # JWT 认证中间件
///
/// 这个中间件用于验证用户的 JWT Token，确保请求来自已认证的用户
///
/// ## 功能：
/// 1. 从请求头中提取 JWT Token
/// 2. 先检查 Redis 缓存中的 Token 是否有效
/// 3. 如果 Redis 中没有，验证 JWT 签名
/// 4. 将用户信息（Claims）存入请求扩展中，供后续中间件和 Handler 使用
///
/// ## 使用方法：
/// ```rust
/// web::scope("/api")
///     .wrap(AuthMiddleware)  // 添加 JWT 认证
///     .route("/users", web::get().to(get_users))
/// ```
///
/// ## 工作流程：
/// 1. 检查请求头中的 Authorization 字段
/// 2. 提取 Bearer Token
/// 3. 从 Redis 检查 Token 是否存在且未过期
/// 4. 如果 Redis 检查通过，继续执行请求
/// 5. 如果 Redis 中没有，验证 JWT 签名
/// 6. 如果验证失败，返回 401 Unauthorized
///
/// ## Token 格式：
/// ```
/// Authorization: Bearer <jwt_token>
/// ```
///
/// ## 注意事项：
/// - 必须在需要认证的路由上使用
/// - 应该在权限中间件（AdminMiddleware、ResourcePermissionMiddleware）之前使用
/// - Token 存储在 Redis 中，登出时会从 Redis 删除
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // ready(Ok(AuthMiddlewareService { service }))
        ready(Ok(AuthMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>> +'static,
    S::Future: 'static,
    S::Error:Into<Error>,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 获取请求头中的认证部分
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].to_string();

                    let config = req.app_data::<web::Data<Config>>().cloned();
                    let redis = req.app_data::<web::Data<ConnectionManager>>().cloned();

                    if let (Some(config), Some(redis)) = (config, redis) {

                        // let fut = self.service.call(req);

                        let service = Rc::clone(&self.service);

                        return Box::pin(async move {
                            // 先检查Redis缓存
                            match redis_cache::get_token(&redis,&token).await {

                                Ok(Some(cache)) => {
                                    // Token 在 Redis 中存在，验证是否过期
                                    let now = chrono::Utc::now().timestamp();
                                    if cache.exp > now {
                                        // Token 有效，直接返回
                                        // let res = fut.await?;
                                        // Ok(res)

                                        // 应该先检查token是否过期，然后再调用服务
                                        service.call(req).await.map_err(Into::into)
                                    } else {
                                        // Token 已过期，从 Redis 删除
                                        let _ = redis_cache::delete_token(&redis, &token).await;
                                        Err(actix_web::error::InternalError::from_response(
                                            "",
                                            ErrorResponse::unauthorized("Token expired"),
                                        ).into())
                                    }
                                }

                                Ok(None) => {
                                    // Token 不在 Redis 中，验证 JWT 签名
                                    match verify_jwt(&token, &config.jwt_secret) {
                                        Ok(_claims) => {
                                            // JWT 有效但不在 Redis，可能是 Redis 重启或 token 被删除
                                            Err(actix_web::error::InternalError::from_response(
                                                "",
                                                ErrorResponse::unauthorized("Token not found in cache"),
                                            ).into())
                                        }
                                        Err(_) => {
                                            Err(actix_web::error::InternalError::from_response(
                                                "",
                                                ErrorResponse::unauthorized("Invalid token"),
                                            ).into())
                                        }
                                    }
                                }

                                Err(_) => {
                                    // Redis 错误，降级到只验证 JWT
                                    match verify_jwt(&token, &config.jwt_secret) {
                                        Ok(_claims) => {
                                            // let res = fut.await?;
                                            // Ok(res)

                                            // 先检查是否有报错，然后再调用服务，更符合业务逻辑
                                            service.call(req).await.map_err(Into::into)
                                        }
                                        Err(_) => {
                                            Err(actix_web::error::InternalError::from_response(
                                                "",
                                                ErrorResponse::unauthorized("Invalid token"),
                                            ).into())
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
            }
        }

        let (_req, _) = req.into_parts();
        let response = ErrorResponse::unauthorized("Missing or invalid authorization header");
        Box::pin(async move {
            Err(actix_web::error::InternalError::from_response("", response).into())
        })

    }
}
