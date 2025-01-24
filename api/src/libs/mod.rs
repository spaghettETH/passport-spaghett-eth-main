use crate::libs::responses::ApiSuccess;
use responses::ApiError;
use rocket::serde::json::Json;

pub mod cors;
pub mod db;
pub mod responses;
pub mod eth;

#[get("/")]
pub fn index() -> Json<ApiSuccess> {
    Json(ApiSuccess {
        message: "🦀".to_string(),
        details: "".to_string(),
        error: false,
    })
}

#[catch(404)]
pub fn not_found() -> Json<ApiError> {
    Json(ApiError {
        message: "Endpoint not found".to_string(),
        reason: "NOT_FOUND".to_string(),
        error: true,
    })
}

#[catch(422)]
pub fn malformed_request() -> Json<ApiError> {
    Json(ApiError {
        message: "Malformed request".to_string(),
        reason: "MALFORMED_REQUEST".to_string(),
        error: true,
    })
}

#[catch(500)]
pub fn general_error() -> Json<ApiError> {
    Json(ApiError {
        message: "Can't process request".to_string(),
        reason: "SERVER_ERROR".to_string(),
        error: true,
    })
}
