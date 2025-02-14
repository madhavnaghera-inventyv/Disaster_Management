use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: bool,
    pub message: String,
    pub data: T,
}

pub fn success_response<T>(message: &str, data: T , status_code : StatusCode) -> Response
where
    T: serde::Serialize,
{
    let response = ApiResponse {
        status: true,
        message: message.to_string(),
        data,
    };
    (status_code, Json(response)).into_response()
}

pub fn error_response(message: &str, status_code: StatusCode) -> Response {
    let response = ApiResponse {
        status: false,
        message: message.to_string(),
        data: Vec::<serde_json::Value>::new(), 
    };
    (status_code, Json(response)).into_response()
}
