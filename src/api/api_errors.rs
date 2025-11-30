use thiserror::Error;

#[derive(Debug, Error,Clone)]
pub enum ApiError {
    #[error("Expired or missiing auth token")]
    NotAuthenticated,
    #[error("Unknow Network error")]
    Unknow,
}
