use std::error::Error;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;

#[derive(Debug, Display)]
#[display("ErrorInfo: status_code: {}, message: {}, error_code: {}, origin: {}", status_code.to_string(), message, error_code, origin.as_ref().unwrap_or(&"None".to_string()))]
pub struct ServerError {
    status_code: StatusCode,
    message: &'static str,
    error_code: u32,
    internal_error: bool,
    origin: Option<String>,
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let mut response_builder = HttpResponse::build(self.status_code());
        println!("Error response: {:?}", self);
        match self.internal_error {
            true => response_builder.json(json!(
            {
                "message": "Internal server error",
                "error_code": 0
            })),
            false => response_builder.json(json!(
            {
                "message": self.message,
                "error_code": self.error_code
            })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self.internal_error {
            true => StatusCode::INTERNAL_SERVER_ERROR,
            false => self.status_code,
        }
    }
}

impl Error for ServerError {}

impl ServerError {
    pub fn invalid_user_credentials() -> ServerError {
        ServerError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Invalid user credentials",
            error_code: 1000,
            internal_error: false,
            origin: None,
        }
    }

    pub fn fail_to_communicate_with_keycloak(origin: &str) -> ServerError {
        ServerError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Fail to communicate with keycloak",
            error_code: 1,
            internal_error: true,
            origin: Some(origin.to_string()),
        }
    }

    pub fn unable_to_parse_response(origin: &str) -> ServerError {
        ServerError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Unable to parse response",
            error_code: 2,
            internal_error: true,
            origin: Some(origin.to_string()),
        }
    }

    pub fn unable_to_decode_token(origin: &str) -> ServerError {
        ServerError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Unable to decode token",
            error_code: 3,
            internal_error: true,
            origin: Some(origin.to_string()),
        }
    }

    pub fn user_already_exists() -> ServerError {
        ServerError {
            status_code: StatusCode::CONFLICT,
            message: "User already exists",
            error_code: 1001,
            internal_error: false,
            origin: None,
        }
    }
}
