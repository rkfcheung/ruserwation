#[derive(Debug)]
pub enum SetupError {
    Database(sqlx::Error), // For sqlx errors
    InvalidConfig(String), // For invalid configuration errors
    IO(std::io::Error),    // For I/O related errors
    Other(String),         // For any other kind of error
}
