use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{CreateProjectRequest, UpdateProjectRequest, AddProjectImageRequest};
use crate::utils::{Claims, ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;
use crate::db;

pub async fn create_project(
    pool: web::Data<MySqlPool>,
    req_http: HttpRequest,
    req: web::Json<CreateProjectRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let claims = req_http.extensions().get::<Claims>().cloned();
    
    if let Some(claims) = claims {
        match db::create_project(&pool, &req, claims.sub).await {
            Ok(project_id) => {
                match db::get_project_by_id(&pool, project_id).await {
                    Ok(Some(project)) => ApiResponse::success_with_message("Project created successfully", project),
                    _ => ErrorResponse::internal_error("Failed to fetch created project"),
                }
            }
            Err(_) => ErrorResponse::internal_error("Database error"),
        }
    } else {
        ErrorResponse::unauthorized("Unauthorized")
    }
}

pub async fn get_all_projects(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_all_projects(&pool).await {
        Ok(projects) => ApiResponse::success(projects),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

/// 获取已发布的项目
pub async fn get_published_projects(pool: web::Data<MySqlPool>) -> HttpResponse {
    match db::get_published_projects(&pool).await {
        Ok(projects) => ApiResponse::success(projects),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_project(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let project_id = path.into_inner();
    
    match db::get_project_by_id(&pool, project_id).await {
        Ok(Some(project)) => ApiResponse::success(project),
        Ok(None) => ErrorResponse::not_found("Project not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn update_project(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
    req: web::Json<UpdateProjectRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    let project_id = path.into_inner();

    match db::update_project(&pool, project_id, &req).await {
        Ok(true) => {
            match db::get_project_by_id(&pool, project_id).await {
                Ok(Some(project)) => ApiResponse::success_with_message("Project updated successfully", project),
                _ => ErrorResponse::internal_error("Failed to fetch updated project"),
            }
        }
        Ok(false) => ErrorResponse::not_found("Project not found or no changes made"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_project(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let project_id = path.into_inner();

    match db::delete_project(&pool, project_id).await {
        Ok(true) => ApiResponse::success_with_message("Project deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("Project not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn add_project_image(
    pool: web::Data<MySqlPool>,
    req: web::Json<AddProjectImageRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return ErrorResponse::bad_request(&format!("Validation error: {:?}", errors));
    }

    match db::add_project_image(&pool, &req).await {
        Ok(_) => ApiResponse::success_with_message("Image added successfully", ()),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn get_project_images(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let project_id = path.into_inner();

    match db::get_project_images(&pool, project_id).await {
        Ok(images) => ApiResponse::success(images),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub async fn delete_project_image(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let image_id = path.into_inner();

    match db::delete_project_image(&pool, image_id).await {
        Ok(true) => ApiResponse::success_with_message("Image deleted successfully", ()),
        Ok(false) => ErrorResponse::not_found("Image not found"),
        Err(_) => ErrorResponse::internal_error("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .route("/published", web::get().to(get_published_projects))
            .route("", web::get().to(get_all_projects))
            .route("/{id}", web::get().to(get_project))
            // .route("/images", web::post().to(add_project_image))
            // .route("/{id}/images", web::get().to(get_project_images))
            // .route("/{id}", web::put().to(update_project))
            // .route("/{id}", web::delete().to(delete_project))

            .service(
                web::scope("")
                    .wrap(AuthMiddleware)
                    .route("", web::post().to(create_project))
                    .route("/{id}", web::put().to(update_project))
                    .route("/{id}", web::delete().to(delete_project))
                    .route("/images", web::post().to(add_project_image))
                    .route("/{id}/images", web::get().to(get_project_images))
                    .route("/images/{id}", web::delete().to(delete_project_image))
            )
    );
}
