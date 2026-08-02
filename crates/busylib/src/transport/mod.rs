//! The HTTP transport which the client sends its requests over

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use http::{Request, Response};

#[cfg(feature = "reqwest")]
mod reqwest_transport;
mod timeout;
#[cfg(feature = "ws")]
mod tungstenite_transport;
mod ws;

#[cfg(feature = "reqwest")]
pub use reqwest_transport::ReqwestHttpTransport;
pub use timeout::Timeout;
#[cfg(feature = "ws")]
pub use tungstenite_transport::TungsteniteWsTransport;
pub use ws::{WsConnection, WsMessage, WsTransport, WsTransportError, WsTransportResult};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type HttpTransportResult = Result<Response<Bytes>, HttpTransportError>;

/// The HTTP transport which the client sends its requests over
pub trait HttpTransport: Send + Sync {
    fn execute(&self, request: Request<Bytes>) -> BoxFuture<'_, HttpTransportResult>;
}

/// Failure from the underlying transport
pub struct HttpTransportError(Box<dyn std::error::Error + Send + Sync>);

impl HttpTransportError {
    pub fn new<E>(source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self(source.into())
    }
}

impl fmt::Debug for HttpTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for HttpTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for HttpTransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl<T: HttpTransport + ?Sized> HttpTransport for &T {
    fn execute(&self, request: Request<Bytes>) -> BoxFuture<'_, HttpTransportResult> {
        (**self).execute(request)
    }
}

impl<T: HttpTransport + ?Sized> HttpTransport for Arc<T> {
    fn execute(&self, request: Request<Bytes>) -> BoxFuture<'_, HttpTransportResult> {
        (**self).execute(request)
    }
}

impl<T: HttpTransport + ?Sized> HttpTransport for Box<T> {
    fn execute(&self, request: Request<Bytes>) -> BoxFuture<'_, HttpTransportResult> {
        (**self).execute(request)
    }
}
