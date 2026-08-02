use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::transport::HttpTransport;
use crate::types::{AppName, AssetName, TryIntoValue};

crate::api::endpoint!(
    /// Asset file management
    Assets
);

impl<T: HttpTransport> Assets<'_, T> {
    /// Upload asset file with app ID
    ///
    /// Uploads a file to a specific app's assets directory
    pub async fn upload(
        &self,
        application_name: impl TryIntoValue<AppName>,
        file: impl TryIntoValue<AssetName>,
        data: impl Into<Bytes>,
    ) -> Result<()> {
        let request = Call::post("/busybar/assets/upload")
            .query("application_name", application_name.try_into_value()?)
            .query("file", file.try_into_value()?)
            .octet_stream(data);
        self.client.ok(request).await
    }

    /// Delete app assets
    ///
    /// Deletes all assets for a specific app ID
    pub async fn delete(&self, application_name: impl TryIntoValue<AppName>) -> Result<()> {
        let request = Call::delete("/busybar/assets/upload")
            .query("application_name", application_name.try_into_value()?);
        self.client.ok(request).await
    }
}
