use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Communication failure: {0}")]
    Network(String),

    #[error("Internal processing error: {0}")]
    Internal(String),

    #[error("Unauthorized access or security violation")]
    Security,

    #[error("Agent failure: {0}")]
    Agent(String),

    #[error("Mission failure: {0}")]
    Mission(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("Database or storage error: {0}")]
    Storage(String),
}

pub type Result<T> = std::result::Result<T, FactoryError>;
