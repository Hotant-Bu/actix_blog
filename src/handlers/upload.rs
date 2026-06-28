use actix_web::{web, HttpResponse};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;
use serde::Serialize;

use crate::config::Config;
use crate::utils::{ApiResponse, ErrorResponse};
use crate::middleware::AuthMiddleware;

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub filename: String,
}

pub async fn upload_file(
    mut payload: Multipart,
    config: web::Data<Config>,
) -> HttpResponse {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return ErrorResponse::bad_request("Failed to read multipart field"),
        };

        let content_disposition = field.content_disposition();
        let original_filename = content_disposition
            .get_filename()
            .unwrap_or("unnamed");

        let extension = PathBuf::from(original_filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("bin")
            .to_string();

        let filename = format!("{}.{}", Uuid::new_v4(), extension);
        let filepath = PathBuf::from(&config.upload_dir).join(&filename);

        let mut file = match std::fs::File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return ErrorResponse::internal_error("Failed to create file"),
        };

        let mut total_size = 0;

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(data) => data,
                Err(_) => return ErrorResponse::bad_request("Failed to read chunk"),
            };

            total_size += data.len();
            if total_size > config.max_file_size {
                let _ = std::fs::remove_file(&filepath);
                return ErrorResponse::bad_request("File size exceeds limit");
            }

            if let Err(_) = file.write_all(&data) {
                let _ = std::fs::remove_file(&filepath);
                return ErrorResponse::internal_error("Failed to write file");
            }
        }

        let url = format!("/uploads/{}", filename);
        let response = UploadResponse {
            url,
            filename,
        };

        return ApiResponse::success_with_message("File uploaded successfully", response);
    }

    ErrorResponse::bad_request("No file provided")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/upload")
            .wrap(AuthMiddleware)
            .route("", web::post().to(upload_file))
    );
}
