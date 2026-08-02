//! Rust client for the BUSY Bar HTTP API

pub mod api;
mod client;
mod error;
pub mod model;
mod serde_util;
mod transport;
pub mod types;

#[cfg(feature = "ws")]
pub use busylib_proto as proto;
pub use client::{ApiPrefix, Client, ClientBuilder};
pub use error::{ApiError, BaseUrlError, Body, BuildRequestError, Error, Result};
pub use transport::{
    BoxFuture, HttpTransport, HttpTransportError, HttpTransportResult, Timeout, WsConnection,
    WsMessage, WsTransport, WsTransportError, WsTransportResult,
};

#[cfg(feature = "reqwest")]
pub use transport::ReqwestHttpTransport;
#[cfg(feature = "ws")]
pub use transport::TungsteniteWsTransport;
