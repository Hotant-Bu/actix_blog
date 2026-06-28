use actix_web::{
    dev::{forward_ready,Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use std::future::{ready, Ready};
use std::rc::Rc;
use futures_util::future::LocalBoxFuture;
use redis::aio::ConnectionManager;
use sqlx::MySqlPool;
use crate::utils::ErrorResponse;

/// 管理员权限中间件
/// 检查用户是否有超级管理员权限
pub struct AdminMiddleware;

/// 管理员中间件服务
/// 是实际执行权限检查的服务结构体
pub struct AdminMiddlewareService<S> {
    /// 被包装的下一个服务（可能是另一个中间件或最终的Handler）
    service: Rc<S>,
}

/// Transform trait 负责将中间件转换为实际的服务
impl<S, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>+'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// 创建中间件服务实例
    ///
    /// 这个方法应该启动时被调用，用于初始化中间件
    fn new_transform(&self, service: S) -> Self::Future {
        // ready(Ok(AdminMiddlewareService{service}))
        ready(Ok(AdminMiddlewareService{service: Rc::new(service)}))
    }
}

/// Service trait 定义了中间件如何处理请求
impl<S,B> Service<ServiceRequest> for AdminMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>> +'static,
    S::Future: 'static,
    S::Error: Into<Error>,
    B: 'static,
{

    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    // 使用宏转发ready，检查到内部服务
    forward_ready!(service);

    /// 处理请求核心方法
    fn call(&self, req: ServiceRequest) -> Self::Future {

        // 从请求扩展中获取Claims
        let claims = req.extensions().get::<crate::utils::Claims>().cloned();

        // 如果Claims存在，说明用户已通过JWT认证
        if let Some(claims) = claims {
            // 从应用数据中获取数据库连接池和Redis链接
            let pool = req.app_data::<actix_web::web::Data<MySqlPool>>().cloned();
            let redis = req.app_data::<actix_web::web::Data<ConnectionManager>>().cloned();

            // 数据库和Redis链接都存在
            if let (Some(pool),Some(redis)) = (pool,redis) { 
                
                // 调用下一个服务（可能是另一个中间件或Handler）
                // let fut = self.service.call(req);

                // 应该先检查权限，再调用服务
                let service = Rc::clone(&self.service);

                // 返回一个异步Future
                return Box::pin(async move {
                    // 检查用户是否有超级管理员权限
                    match crate::utils::permission::check_permission(
                        &pool,
                        &redis,
                        // 用户ID
                        claims.sub,
                        // 资源，所有资源
                        "*",
                        // 操作：所有操作
                        "*"
                    ).await {
                        // 情况1: 用户有超级管理员权限
                        Ok(true) => {
                            // 继续执行请求，调用下一个服务
                            // let res = fut.await.map_err(Into::into);
                            // Ok(res)

                            // 应该先检查权限，再调用服务
                            service.call(req).await.map_err(Into::into)

                        }
                        // 情况2: 用户没有超级管理员权限
                        Ok(false) => {
                            // 返回 403 Forbidden
                            Err(actix_web::error::InternalError::from_response(
                                "",
                                ErrorResponse::forbidden("Super admin access required"),
                            ).into())
                        }
                        // 情况3: 权限检查失败（数据库或 Redis 错误）
                        Err(_) => {
                            // 返回 500 Internal Server Error
                            Err(actix_web::error::InternalError::from_response(
                                "",
                                ErrorResponse::internal_error("Failed to check permissions"),
                            ).into())
                        }
                    }
                });
            }
        }

        // 如果没有 Claims，说明用户未认证
        // 销毁请求对象，返回 401 Unauthorized
        let (_req, _) = req.into_parts();
        let response = ErrorResponse::unauthorized("Authentication required");
        Box::pin(async move {
            Err(actix_web::error::InternalError::from_response("", response).into())
        })
    }
}

/// 资源权限中间件
pub struct ResourcePermissionMiddleware{
    // 资源名称
    pub resource: String,
    // 操作名称
    pub action: String,
}

pub struct ResourcePermissionMiddlewareService<S> {
    service: Rc<S>,
    resource: String,
    action: String,
}

impl ResourcePermissionMiddleware {
    pub fn new(resource: &str, action: &str) -> Self {
        Self{
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }
}

impl<S,B> Transform<S, ServiceRequest> for ResourcePermissionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> +'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ResourcePermissionMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ResourcePermissionMiddlewareService{
            service: Rc::new(service),
            resource: self.resource.clone(),
            action: self.action.clone(),
        }))
    }
}

impl<S,B> Service<ServiceRequest> for ResourcePermissionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>> +'static,
    S::Future: 'static,
    S::Error: Into<Error>,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let claims = req.extensions().get::<crate::utils::Claims>().cloned();

        if let Some(claims) = claims {
            let pool = req.app_data::<actix_web::web::Data<MySqlPool>>().cloned();
            let redis = req.app_data::<actix_web::web::Data<ConnectionManager>>().cloned();

            if let (Some(redis), Some(pool)) = (redis, pool) {
                // let fut = self.service.call(req);
                // 应该先检查权限，再调用服务
                let service = Rc::clone(&self.service);

                let resource = self.resource.clone();
                let action = self.action.clone();
                return Box::pin(async move {

                    match crate::utils::permission::check_permission(
                        &pool,
                        &redis,
                        claims.sub,
                        &resource,
                        &action
                    ).await {
                        Ok(true) => {
                            // let res = fut.await?;
                            // Ok(res)

                            // 应该先检查权限再调用服务
                            service.call(req).await.map_err(Into::into)

                        }
                        Ok(false) => {
                            Err(actix_web::error::InternalError::from_response(
                                "",
                                ErrorResponse::forbidden(&format!(
                                    "Permission required: {}:{}",
                                    resource, action
                                )),
                            ).into())
                        }
                        Err(_) => {
                            Err(actix_web::error::InternalError::from_response(
                                "",
                                ErrorResponse::internal_error("Failed to check permissions"),
                            ).into())
                        }
                    }
                });
            }
        }
        let (_req, _) = req.into_parts();
        let response = ErrorResponse::unauthorized("Authentication required");
        Box::pin(async move {
            Err(actix_web::error::InternalError::from_response("", response).into())
        })
    }
}






