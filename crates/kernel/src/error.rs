//! Error types for Delta Lake kernel operations.

use super::DataType;

/// A specialized [`Result`] type for Delta Lake kernel operations.
pub type KernelResult<T> = std::result::Result<T, KernelError>;

/// Errors that can occur in the Delta Lake kernel
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum KernelError {
    #[error("Protocol error: {0}")]
    Protocol(#[from] deltalake_protocol::ProtocolError),

    #[error("Delta kernel error: {0}")]
    DeltaKernel(#[from] delta_kernel::error::Error),

    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow_schema::ArrowError),

    #[error("Parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),

    #[error("Object store error: {0}")]
    ObjectStore(#[from] object_store::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("{0}")]
    MissingColumn(String),

    #[error("Expected column type: {0}")]
    UnexpectedColumnType(String),

    #[error("Expected data is missing: {0}")]
    MissingData(String),

    #[error("No table version found.")]
    MissingVersion,

    #[error("Deletion Vector error: {0}")]
    DeletionVector(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Invalid url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Invalid JSON: {0}")]
    MalformedJson(#[from] serde_json::Error),

    #[error("No table metadata found in delta log.")]
    MissingMetadata,

    /// Error returned when the log contains invalid stats JSON.
    #[error("Invalid JSON in invariant expression, line=`{line}`, err=`{json_err}`")]
    InvalidInvariantJson {
        /// JSON error details returned when parsing the invariant expression JSON.
        json_err: serde_json::error::Error,
        /// Invariant expression.
        line: String,
    },

    /// Error returned when the log contains invalid stats JSON.
    #[error("Invalid JSON in generation expression, line=`{line}`, err=`{json_err}`")]
    InvalidGenerationExpressionJson {
        /// JSON error details returned when parsing the generation expression JSON.
        json_err: serde_json::error::Error,
        /// Generation expression.
        line: String,
    },

    #[error("Table metadata is invalid: {0}")]
    MetadataError(String),

    #[error("Failed to parse value '{0}' as '{1}'")]
    Parse(String, DataType),

    #[error("Generic kernel error: {0}")]
    Generic(String),

    #[error("Generic error: {source}")]
    GenericError {
        /// Source error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

// Keep backwards compatibility aliases
pub type DeltaResult<T> = KernelResult<T>;
pub type Error = KernelError;
