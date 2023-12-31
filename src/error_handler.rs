use actix_web::{ResponseError, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CustomError {
    DbError(String),
    QueryError(String),
}



use std::fmt;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::DbError(e) => write!(f, "Database error: {}", e),
            CustomError::QueryError(e) => write!(f, "Query error: {}", e),
        }
    }
}


impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DbError(_) => HttpResponse::InternalServerError().json("Database error"),
            CustomError::QueryError(_) => HttpResponse::BadRequest().json("Query error"),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<diesel::result::Error> for CustomError {
    fn from(error: diesel::result::Error) -> Self {
        CustomError::DbError(error.to_string())
    }
}