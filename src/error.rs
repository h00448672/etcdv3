//! Etcd Client Error handling.

use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type for `etcd` client.
#[derive(Debug)]
pub enum Error {
    /// Invalid arguments
    InvalidArgs(String),

    /// Invalid URI
    InvalidUri(http::uri::InvalidUri),

    /// IO error
    IOError(std::io::Error),

    /// Transport error
    TransportError(tonic::transport::Error),

    /// gRPC status
    GRPCStatus(tonic::Status),

    /// Watch error
    WatchError(String),

    /// Utf8Error
    Utf8Error(Utf8Error),

    /// Lease error
    LeaseKeepAliveError(String),

    /// Election error
    ElectError(String),

    /// Invalid metadata value
    InvalidMetadataValue(tonic::metadata::errors::InvalidMetadataValue),
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgs(e) => write!(f, "invalid arguments: {}", e),
            Error::InvalidUri(e) => write!(f, "invalid uri: {}", e),
            Error::IOError(e) => write!(f, "io error: {}", e),
            Error::TransportError(e) => write!(f, "transport error: {}", e),
            Error::GRPCStatus(e) => write!(f, "grep request error: {}", e),
            Error::WatchError(e) => write!(f, "watch error: {}", e),
            Error::Utf8Error(e) => write!(f, "utf8 error: {}", e),
            Error::LeaseKeepAliveError(e) => write!(f, "lease keep alive error: {}", e),
            Error::ElectError(e) => write!(f, "election error: {}", e),
            Error::InvalidMetadataValue(e) => write!(f, "invalid metadata value: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<http::uri::InvalidUri> for Error {
    #[inline]
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::InvalidUri(e)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<tonic::transport::Error> for Error {
    #[inline]
    fn from(e: tonic::transport::Error) -> Self {
        Error::TransportError(e)
    }
}

impl From<tonic::Status> for Error {
    #[inline]
    fn from(e: tonic::Status) -> Self {
        Error::GRPCStatus(e)
    }
}

impl From<Utf8Error> for Error {
    #[inline]
    fn from(e: Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}

impl From<tonic::metadata::errors::InvalidMetadataValue> for Error {
    #[inline]
    fn from(e: tonic::metadata::errors::InvalidMetadataValue) -> Self {
        Error::InvalidMetadataValue(e)
    }
}
