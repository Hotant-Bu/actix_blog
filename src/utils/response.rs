use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Success".to_string(),
            data: Some(data),
        })
    }

    pub fn success_with_message(message: &str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: message.to_string(),
            data: Some(data),
        })
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: &str, status: StatusCode) -> HttpResponse {
        HttpResponse::build(status).json(ErrorResponse {
            success: false,
            message: message.to_string(),
        })
    }

    pub fn bad_request(message: &str) -> HttpResponse {
        Self::new(message, StatusCode::BAD_REQUEST)
    }

    pub fn unauthorized(message: &str) -> HttpResponse {
        Self::new(message, StatusCode::UNAUTHORIZED)
    }

    pub fn forbidden(message: &str) -> HttpResponse {
        Self::new(message, StatusCode::FORBIDDEN)
    }

    pub fn not_found(message: &str) -> HttpResponse {
        Self::new(message, StatusCode::NOT_FOUND)
    }

    pub fn internal_error(message: &str) -> HttpResponse {
        Self::new(message, StatusCode::INTERNAL_SERVER_ERROR)
    }
}
