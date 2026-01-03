use actix_web::{HttpResponse, ResponseError};
use crate::models::responses::AppError;
use crate::models::responses::Response;

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden => actix_web::http::StatusCode::FORBIDDEN,
            AppError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            AppError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        HttpResponse::build(status).json(Response {
            success: false,
            message: self.to_string(),
            code: status.as_u16(),
            data: None,
            description: self.source(),
            status: String::from("Error"),
        })
    }
}