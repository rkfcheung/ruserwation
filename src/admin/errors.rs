use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SessionError {
    #[error("Failed to create session for '{0}'")]
    SessionCreationFailed(String),

    #[error("Session '{0}' has expired")]
    SessionExpired(String),

    #[error("Session '{0}' not found")]
    SessionNotFound(String),

    #[error("Store exception: '{0}'")]
    StoreException(String),

    #[error("User '{0}' not found")]
    UserNotFound(String),
}
