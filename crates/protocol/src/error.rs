//! Error types for the Delta Lake protocol

use object_store::Error as ObjectStoreError;

/// Result type for protocol operations
pub type ProtocolResult<T> = Result<T, ProtocolError>;

/// Errors that can occur when working with Delta Lake protocol types
#[derive(thiserror::Error, Debug)]
pub enum ProtocolError {
    /// Error returned when serializing/deserializing JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic protocol error
    #[error("Protocol error: {0}")]
    Generic(String),
}

/// Error raised while committing transaction
#[derive(thiserror::Error, Debug)]
pub enum TransactionError {
    /// Version already exists
    #[error("Tried committing existing table version: {0}")]
    VersionAlreadyExists(i64),

    /// Error returned when reading the delta log object failed.
    #[error("Error serializing commit log to json: {json_err}")]
    SerializeLogJson {
        /// Commit log record JSON serialization error.
        json_err: serde_json::error::Error,
    },

    /// Error returned when reading the delta log object failed.
    #[error("Log storage error: {}", .source)]
    ObjectStore {
        /// Storage error details when reading the delta log object failed.
        #[from]
        source: ObjectStoreError,
    },

    /// Error returned when a commit conflict occurred
    #[error("Failed to commit transaction: {0}")]
    CommitConflict(String),

    /// Error returned when maximum number of commit trials is exceeded
    #[error("Failed to commit transaction: {0}")]
    MaxCommitAttempts(i32),

    /// The transaction includes Remove action with data change but Delta table is append-only
    #[error(
        "The transaction includes Remove action with data change but Delta table is append-only"
    )]
    DeltaTableAppendOnly,

    /// Error returned when unsupported reader features are required
    #[error("Unsupported reader features required: {0:?}")]
    UnsupportedReaderFeatures(Vec<delta_kernel::table_features::ReaderFeature>),

    /// Error returned when unsupported writer features are required
    #[error("Unsupported writer features required: {0:?}")]
    UnsupportedWriterFeatures(Vec<delta_kernel::table_features::WriterFeature>),

    /// Error returned when writer features are required but not specified
    #[error("Writer features must be specified for writerversion >= 7, please specify: {0:?}")]
    WriterFeaturesRequired(delta_kernel::table_features::WriterFeature),
}

