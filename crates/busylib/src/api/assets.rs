use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::transport::HttpTransport;
use crate::types::{AppName, AssetName, TryIntoValue};

crate::api::endpoint!(Assets);

impl<T: HttpTransport> Assets<'_, T> {
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

    pub async fn delete(&self, application_name: impl TryIntoValue<AppName>) -> Result<()> {
        let request = Call::delete("/busybar/assets/upload")
            .query("application_name", application_name.try_into_value()?);
        self.client.ok(request).await
    }
}
