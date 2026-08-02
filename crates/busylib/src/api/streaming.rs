//! Streaming endpoints

use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::assets::Screen;
use crate::transport::HttpTransport;

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

    /// Device status streaming WebSocket endpoint
    ///
    /// Not implemented yet: the endpoint upgrades to a WebSocket, which cannot travel through
    /// [`HttpTransport`] currently. Panics when called. Included only for completeness, so i dont forget to impl it later sake..
    pub async fn status_ws(&self) -> Result<()> {
        todo!("/status/ws needs a websocket transport")
    }
}
