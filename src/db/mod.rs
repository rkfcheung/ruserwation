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
    NoConditionsProvided,
    NotFound(String),
    SqlxError(BoxDynError),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::NoConditionsProvided => write!(f, "No conditions provided for the query."),
            QueryError::NotFound(err) => write!(f, "Not found for the query: {}", err),
            QueryError::SqlxError(err) => write!(f, "SQLx error: {}", err),
        }
    }
}

impl PartialEq for QueryError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (QueryError::NoConditionsProvided, QueryError::NoConditionsProvided) => true,
            (QueryError::NotFound(a), QueryError::NotFound(b)) => a == b,
            (QueryError::SqlxError(a), QueryError::SqlxError(b)) => a.to_string() == b.to_string(),
            _ => false,
        }
    }
}

impl From<BoxDynError> for QueryError {
    fn from(err: BoxDynError) -> Self {
        QueryError::SqlxError(err)
    }
}
