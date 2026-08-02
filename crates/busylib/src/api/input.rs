//! Input endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::input::Key;
use crate::transport::HttpTransport;

crate::api::endpoint!(
    /// Input events
    Input
);

impl<T: HttpTransport> Input<'_, T> {
    /// Send input event
    ///
    /// Send single key press event
    pub async fn press(&self, key: Key) -> Result<()> {
        let request = Call::post("input").query("key", key.as_str());
        self.client.ok(request).await
    }
}
