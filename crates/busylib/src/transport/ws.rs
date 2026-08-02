use std::fmt;
use std::sync::Arc;

use bytes::Bytes;
use http::{Request, StatusCode};

use super::BoxFuture;

pub type WsTransportResult = Result<Box<dyn WsConnection>, WsTransportError>;

pub trait WsTransport: Send + Sync {
    fn connect(&self, request: Request<()>) -> BoxFuture<'_, WsTransportResult>;
}

pub trait WsConnection: Send {
    fn send(&mut self, message: WsMessage) -> BoxFuture<'_, Result<(), WsTransportError>>;

    fn recv(&mut self) -> BoxFuture<'_, Option<Result<WsMessage, WsTransportError>>>;

    fn close(&mut self) -> BoxFuture<'_, Result<(), WsTransportError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WsMessage {
    Text(String),
    Binary(Bytes),
}

pub struct WsTransportError {
    status: Option<StatusCode>,
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl WsTransportError {
    pub fn new<E>(source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            status: None,
            source: source.into(),
        }
    }

    pub fn rejected<E>(status: StatusCode, source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            status: Some(status),
            source: source.into(),
        }
    }

    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }
}

impl fmt::Debug for WsTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.source, f)
    }
}

impl fmt::Display for WsTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source, f)
    }
}

impl std::error::Error for WsTransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

impl<T: WsTransport + ?Sized> WsTransport for &T {
    fn connect(&self, request: Request<()>) -> BoxFuture<'_, WsTransportResult> {
        (**self).connect(request)
    }
}

impl<T: WsTransport + ?Sized> WsTransport for Arc<T> {
    fn connect(&self, request: Request<()>) -> BoxFuture<'_, WsTransportResult> {
        (**self).connect(request)
    }
}

impl<T: WsTransport + ?Sized> WsTransport for Box<T> {
    fn connect(&self, request: Request<()>) -> BoxFuture<'_, WsTransportResult> {
        (**self).connect(request)
    }
}
