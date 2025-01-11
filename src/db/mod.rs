use std::fmt;

use sqlx::error::BoxDynError;

pub mod sqlite;

pub enum OpType {
    Insert,
    Update,
    NoOp,
}

#[derive(Debug)]
pub enum QueryError {
    NoConditionsProvided,
    SqlxError(BoxDynError),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::NoConditionsProvided => write!(f, "No conditions provided for the query."),
            QueryError::SqlxError(err) => write!(f, "SQLx error: {}", err),
        }
    }
}

impl From<BoxDynError> for QueryError {
    fn from(err: BoxDynError) -> Self {
        QueryError::SqlxError(err)
    }
}
