use bytes::Bytes;
use http::{Request, Response};

use super::{BoxFuture, HttpTransport, HttpTransportError, HttpTransportResult, Timeout};

#[derive(Debug, Clone, Default)]
pub struct ReqwestHttpTransport {
    client: reqwest::Client,
}

impl ReqwestHttpTransport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

impl HttpTransport for ReqwestHttpTransport {
    fn execute(&self, request: Request<Bytes>) -> BoxFuture<'_, HttpTransportResult> {
        let client = self.client.clone();
        let timeout = Timeout::of(&request);
        Box::pin(async move {
            let (parts, body) = request.into_parts();
            let mut builder = client
                .request(parts.method, parts.uri.to_string())
                .headers(parts.headers)
                .body(body);
            if let Some(timeout) = timeout {
                builder = builder.timeout(timeout);
            }

            let response = builder.send().await.map_err(HttpTransportError::new)?;

            let mut builder = Response::builder().status(response.status());

            if let Some(headers) = builder.headers_mut() {
                *headers = response.headers().clone();
            }

            let body = response.bytes().await.map_err(HttpTransportError::new)?;
            builder.body(body).map_err(HttpTransportError::new)
        })
    }
}
