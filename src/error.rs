use thiserror::Error;
use worker::Error;

/// Trait to convert `Result<T, AppError>` or `anyhow::Result<T>` into `worker::Result<T>`
pub trait IntoWorkerError<T> {
    fn into_worker_error(self) -> worker::Result<T>;
}

impl<T> IntoWorkerError<T> for anyhow::Result<T> {
    fn into_worker_error(self) -> worker::Result<T> {
        self.map_err(|e| Error::from(e.to_string()))
    }
}

impl<T> IntoWorkerError<T> for Result<T, AppError> {
    fn into_worker_error(self) -> worker::Result<T> {
        self.map_err(|e| Error::from(e.to_string()))
    }
}

/// Central application-level error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Discord API error: {0}")]
    DiscordApi(String),

    #[error("JWT error: {0}")]
    Jwt(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Key generation error: {0}")]
    Keygen(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<AppError> for Error {
    fn from(err: AppError) -> Self {
        Error::from(err.to_string())
    }
}
