//! Streaming endpoints

use bytes::Bytes;
use http::Method;
use prost::Message as _;

use crate::client::Call;
use crate::error::{Body, Error, Result};
use crate::model::assets::Screen;
use crate::model::streaming::StreamControl;
use crate::proto::bsb_state::State;
use crate::transport::{HttpTransport, WsConnection, WsMessage, WsTransport};

crate::api::endpoint!(
    /// Screen frames and device status streaming
    Streaming
);

impl<T: HttpTransport> Streaming<'_, T> {
    /// Get single frame for requested screen
    ///
    /// The device answers with the frame encoded as base64, despite announcing `image/bmp`,
    /// and the decoded bytes are raw pixels rather than a BMP file.
    pub async fn screen(&self, screen: Screen) -> Result<Bytes> {
        let request = Call::get("screen").query("display", screen.index());
        self.client.bytes(request).await
    }

    pub async fn status_ws(&self, ws: &impl WsTransport) -> Result<StatusStream> {
        let path = self.client.resolve_path("status/ws");

        let request = self
            .client
            .ws_request(&path)
            .map_err(|source| Error::BuildRequest {
                method: Method::GET,
                path: path.clone(),
                source,
            })?;

        let connection = ws.connect(request).await.map_err(|source| Error::Ws {
            path: path.clone(),
            source,
        })?;

        let mut stream = StatusStream { connection, path };
        stream.enable().await?;

        Ok(stream)
    }
}

pub struct StatusStream {
    connection: Box<dyn WsConnection>,
    path: String,
}

impl StatusStream {
    pub async fn next(&mut self) -> Option<Result<State>> {
        match self.connection.recv().await? {
            Ok(WsMessage::Binary(data)) => Some(self.decode(&data)),
            Ok(WsMessage::Text(text)) => Some(Err(Error::UnexpectedMessage {
                path: self.path.clone(),
                body: Body::new(text.as_bytes()),
            })),
            Err(source) => Some(Err(Error::Ws {
                path: self.path.clone(),
                source,
            })),
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.connection.close().await.map_err(|source| Error::Ws {
            path: self.path.clone(),
            source,
        })
    }

    async fn enable(&mut self) -> Result<()> {
        self.control(StreamControl::enable()).await
    }

    pub async fn disable(&mut self) -> Result<()> {
        self.control(StreamControl::disable()).await
    }

    async fn control(&mut self, control: StreamControl) -> Result<()> {
        let payload = serde_json::to_string(&control).map_err(|source| Error::BuildRequest {
            method: Method::GET,
            path: self.path.clone(),
            source: source.into(),
        })?;

        self.connection
            .send(WsMessage::Text(payload))
            .await
            .map_err(|source| Error::Ws {
                path: self.path.clone(),
                source,
            })
    }

    fn decode(&self, data: &[u8]) -> Result<State> {
        State::decode(data).map_err(|source| Error::DecodeProto {
            path: self.path.clone(),
            body: Body::new(data),
            source,
        })
    }
}

impl std::fmt::Debug for StatusStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusStream")
            .field("path", &self.path)
            .finish()
    }
}
