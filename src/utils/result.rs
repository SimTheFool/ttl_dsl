#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Str(&'static str),
    #[error("{0}")]
    String(String),
}

impl AppError {
    pub fn other<T>(e: T) -> AppError
    where
        T: Into<anyhow::Error>,
    {
        AppError::Other(e.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;
