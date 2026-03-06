use thiserror::Error;

pub type Result<T> = std::result::Result<T, RpcError>;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("request failed: {0}")]
    RequestFailed(String),
    #[error("rpc error {code}: {message}")]
    RpcResponseError { code: i32, message: String },
    #[error("invalid response: {0}")]
    InvalidResponse(String),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}
