use derive_builder::UninitializedFieldError;

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

impl From<UninitializedFieldError> for AppError {
    fn from(ufe: UninitializedFieldError) -> AppError {
        AppError::String(ufe.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
