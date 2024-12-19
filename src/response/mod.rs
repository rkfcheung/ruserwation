use serde::Serialize;
use serde_json::json as to_json;
use warp::{
    filters::body::BodyDeserializeError,
    http::StatusCode,
    reject,
    reply::{json, with_status},
    Rejection, Reply,
};

#[derive(Serialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Serialize)]
pub struct Response {
    message: String,
    status: ResponseStatus,
}

impl Response {
    pub fn ok(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            status: ResponseStatus::Success,
        }
    }

    pub fn err(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            status: ResponseStatus::Error,
        }
    }

    pub fn err_with_code(code: StatusCode) -> Self {
        Self::err(
            code.canonical_reason()
                .unwrap_or("Unknown Http Status Code"),
        )
    }
}

// Define a centralised error handler
pub async fn handle_rejections(err: Rejection) -> Result<impl Reply, Rejection> {
    let response;
    let status;

    if err.is_not_found() {
        // Handle 404 errors
        response = Response::err_with_code(StatusCode::NOT_FOUND);
        status = StatusCode::NOT_FOUND;
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        // Handle deserialisation errors
        response = Response::err(&e.to_string());
        status = StatusCode::BAD_REQUEST;
    } else if let Some(e) = err.find::<reject::MethodNotAllowed>() {
        // Handle method not allowed errors
        response = Response::err(&e.to_string());
        status = StatusCode::METHOD_NOT_ALLOWED;
    } else {
        // Fallback for unhandled rejections
        response = Response::err_with_code(StatusCode::INTERNAL_SERVER_ERROR);
        status = StatusCode::INTERNAL_SERVER_ERROR;
    }

    Ok(with_status(json(&to_json!(response)), status))
}
