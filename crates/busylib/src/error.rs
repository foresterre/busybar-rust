//! Errors returned by the client

use std::fmt;

use http::{Method, StatusCode};
use serde::Deserialize;

use crate::transport::{HttpTransportError, WsTransportError};
use crate::types::invalid_value::InvalidValue;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Busylib's top level error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid base URL `{url}`")]
    BaseUrl {
        url: String,
        #[source]
        source: BaseUrlError,
    },

    #[error(transparent)]
    Value(#[from] InvalidValue),

    #[error("{method} {path} unable to build request")]
    BuildRequest {
        method: Method,
        path: String,
        #[source]
        source: BuildRequestError,
    },

    #[error("{method} {path} unable to reach device")]
    Transport {
        method: Method,
        path: String,
        #[source]
        source: HttpTransportError,
    },

    #[error("{method} {path} was rejected with {status}: {error}")]
    Api {
        method: Method,
        path: String,
        status: StatusCode,
        error: ApiError,
    },

    #[error("{method} {path} returned an unexpected {status} response: {body}")]
    UnexpectedStatus {
        method: Method,
        path: String,
        status: StatusCode,
        body: Body,
    },

    #[cfg(feature = "ws")]
    #[error("{path} sent a message which is not a valid protobuf State: {body}")]
    DecodeProto {
        path: String,
        body: Body,
        #[source]
        source: prost::DecodeError,
    },

    #[error("{path} sent a text message where a protobuf one was expected: {body}")]
    UnexpectedMessage { path: String, body: Body },

    #[error("{path} websocket stream failed")]
    Ws {
        path: String,
        #[source]
        source: WsTransportError,
    },

    #[error("{method} {path} returned a response that does not match the API: {body}")]
    Decode {
        method: Method,
        path: String,
        body: Body,
        #[source]
        source: serde_json::Error,
    },
}

impl Error {
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Error::Api { status, .. } | Error::UnexpectedStatus { status, .. } => Some(*status),
            Error::Ws { source, .. } => source.status(),
            _ => None,
        }
    }

    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            Error::Api { error, .. } => Some(error),
            _ => None,
        }
    }

    pub fn is_status(&self, status: StatusCode) -> bool {
        self.status() == Some(status)
    }

    pub fn is_unauthorized(&self) -> bool {
        self.is_status(StatusCode::UNAUTHORIZED) || self.is_status(StatusCode::FORBIDDEN)
    }

    pub fn is_not_found(&self) -> bool {
        self.is_status(StatusCode::NOT_FOUND)
    }
}

/// Why a base URL is unusable
#[derive(Debug, thiserror::Error)]
pub enum BaseUrlError {
    #[error(transparent)]
    Malformed(#[from] url::ParseError),
    #[error("expected an `http` or `https` URL, got `{0}`")]
    UnsupportedScheme(String),
    #[error("expected a URL with a host")]
    MissingHost,
    #[error("expected a URL without a query or fragment")]
    HasQueryOrFragment,
}

/// Why a request could not be built
#[derive(Debug, thiserror::Error)]
pub enum BuildRequestError {
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Uri(#[from] http::uri::InvalidUri),
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error("could not serialize the request body")]
    Body(#[from] serde_json::Error),
    #[error("could not derive a websocket URL from `{0}`")]
    WsScheme(String),
}

/// Error reported by the device
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub code: Option<i64>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "{} (code {code})", self.error),
            None => write!(f, "{}", self.error),
        }
    }
}

const SNIPPET_LIMIT: usize = 256;

/// The body of a response, specifivally for error messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body {
    text: String,
    truncated: bool,
}

impl Body {
    pub(crate) fn new(body: &[u8]) -> Self {
        let text = String::from_utf8_lossy(body);

        let end = text
            .char_indices()
            .map(|(index, character)| index + character.len_utf8())
            .take_while(|end| *end <= SNIPPET_LIMIT)
            .last()
            .unwrap_or(0);

        Self {
            truncated: end < text.len(),
            text: text[..end].replace(['\n', '\r'], " "),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn is_truncated(&self) -> bool {
        self.truncated
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.text.is_empty() {
            return f.write_str("<empty body>");
        }
        write!(f, "{}", self.text)?;
        if self.truncated {
            f.write_str("...")?;
        }
        Ok(())
    }
}
