use futures_util::{SinkExt as _, StreamExt as _};
use http::Request;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::{Error as TungsteniteError, Message};

use super::{BoxFuture, WsConnection, WsMessage, WsTransport, WsTransportError, WsTransportResult};

#[derive(Debug, Clone, Default)]
pub struct TungsteniteWsTransport;

impl TungsteniteWsTransport {
    pub fn new() -> Self {
        Self
    }
}

impl WsTransport for TungsteniteWsTransport {
    fn connect(&self, request: Request<()>) -> BoxFuture<'_, WsTransportResult> {
        Box::pin(async move {
            let handshake = handshake_request(request)?;

            let (stream, _response) = tokio_tungstenite::connect_async(handshake)
                .await
                .map_err(connect_error)?;

            Ok(Box::new(TungsteniteConnection { stream }) as Box<dyn WsConnection>)
        })
    }
}

fn handshake_request(request: Request<()>) -> Result<Request<()>, WsTransportError> {
    let (parts, ()) = request.into_parts();

    let mut handshake = parts
        .uri
        .into_client_request()
        .map_err(WsTransportError::new)?;

    handshake.headers_mut().extend(parts.headers);

    Ok(handshake)
}

fn connect_error(error: TungsteniteError) -> WsTransportError {
    match error {
        TungsteniteError::Http(response) => {
            let status = response.status();
            WsTransportError::rejected(
                status,
                format!("the device answered the upgrade with {status}"),
            )
        }
        other => WsTransportError::new(other),
    }
}

struct TungsteniteConnection<S> {
    stream: WebSocketStream<S>,
}

impl<S> WsConnection for TungsteniteConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    fn send(&mut self, message: WsMessage) -> BoxFuture<'_, Result<(), WsTransportError>> {
        Box::pin(async move {
            let message = match message {
                WsMessage::Text(text) => Message::text(text),
                WsMessage::Binary(data) => Message::binary(data),
            };

            self.stream
                .send(message)
                .await
                .map_err(WsTransportError::new)
        })
    }

    fn recv(&mut self) -> BoxFuture<'_, Option<Result<WsMessage, WsTransportError>>> {
        Box::pin(async move {
            loop {
                return match self.stream.next().await? {
                    Ok(Message::Text(text)) => Some(Ok(WsMessage::Text(text.as_str().to_owned()))),
                    Ok(Message::Binary(data)) => Some(Ok(WsMessage::Binary(data))),
                    Ok(Message::Ping(_) | Message::Pong(_) | Message::Frame(_)) => continue,
                    Ok(Message::Close(_)) => None,
                    Err(TungsteniteError::ConnectionClosed) => None,
                    Err(error) => Some(Err(WsTransportError::new(error))),
                };
            }
        })
    }

    fn close(&mut self) -> BoxFuture<'_, Result<(), WsTransportError>> {
        Box::pin(async move {
            match self.stream.close(None).await {
                Ok(()) | Err(TungsteniteError::ConnectionClosed) => Ok(()),
                Err(error) => Err(WsTransportError::new(error)),
            }
        })
    }
}
