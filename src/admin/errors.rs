use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SessionError {
    #[error("Failed to create session for '{0}'")]
    SessionCreationFailed(String),

    #[error("Session '{0}' not found")]
    SessionNotFound(String),

    #[error("User '{0}' not found")]
    UserNotFound(String),
}
