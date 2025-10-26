//! Error types for Delta Lake log store operations.

/// A specialized [`Result`] type for Delta Lake log store operations.
pub type LogStoreResult<T> = std::result::Result<T, LogStoreError>;

/// Errors that can occur in the Delta Lake log store
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum LogStoreError {
    #[error("Protocol error: {0}")]
    Protocol(#[from] deltalake_protocol::ProtocolError),

    #[error("Transaction error: {0}")]
    Transaction(#[from] deltalake_protocol::TransactionError),

    #[error("Object store error: {0}")]
    ObjectStore(#[from] object_store::Error),

    #[error("Delta kernel error: {0}")]
    DeltaKernel(#[from] delta_kernel::error::Error),

    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Invalid table location: {0}")]
    InvalidTableLocation(String),

    #[error("Invalid JSON in log: {json_err}, line: {line}, version: {version}")]
    InvalidJsonLog {
        json_err: serde_json::Error,
        line: String,
        version: i64,
    },

    #[error("Generic log store error: {0}")]
    Generic(String),
}

