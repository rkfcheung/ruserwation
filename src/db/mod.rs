use sqlx::error::BoxDynError;
use std::fmt;

pub mod sqlite;

pub enum OpType {
    Insert,
    Update,
    NoOp,
}

#[derive(Debug)]
pub enum QueryError {
    InvalidQuery(String),
    NoConditionsProvided,
    NotFound(String),
    SqlxError(BoxDynError),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::InvalidQuery(err) => write!(f, "Invalid query: {}", err),
            QueryError::NoConditionsProvided => write!(f, "No conditions provided for the query."),
            QueryError::NotFound(err) => write!(f, "Not found for the query: {}", err),
            QueryError::SqlxError(err) => write!(f, "Error: {}", err),
        }
    }
}

impl PartialEq for QueryError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (QueryError::InvalidQuery(a), QueryError::InvalidQuery(b)) => a == b,
            (QueryError::NoConditionsProvided, QueryError::NoConditionsProvided) => true,
            (QueryError::NotFound(a), QueryError::NotFound(b)) => a == b,
            (QueryError::SqlxError(a), QueryError::SqlxError(b)) => a.to_string() == b.to_string(),
            _ => false,
        }
    }
}

impl From<BoxDynError> for QueryError {
    fn from(value: BoxDynError) -> Self {
        QueryError::SqlxError(value)
    }
}

impl From<sqlx::Error> for QueryError {
    fn from(value: sqlx::Error) -> Self {
        QueryError::SqlxError(value.into())
    }
}
