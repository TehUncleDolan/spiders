use std::path::PathBuf;
use thiserror::Error;

/// A specialized Result type for scraping operations.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that may occur while scraping a website.
#[derive(Debug, Error)]
pub enum Error {
    /// Error while querying the website.
    #[error("network request failed for {url}")]
    Network {
        /// Requested URL.
        url: String,
    },

    /// Error while decoding the received payload.
    #[error("received invalid payload from {url}")]
    Payload {
        /// Origin URL of the payload.
        url: String,
    },

    /// Error while scraping payload (HTML, JSON, …).
    #[error("scraping failed: {0}")]
    Scraping(String),

    /// Error while interacting with the filesystem.
    #[error("I/O operation failed: {operation} {target}")]
    Filesystem {
        /// I/O operation.
        operation: &'static str,
        /// Target of the I/O operation.
        target: PathBuf,
        /// Underlying error.
        source: std::io::Error,
    },
}
