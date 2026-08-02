use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::{AutoupdateSettings, ChangelogResponse, UpdateStatus};
use crate::transport::HttpTransport;

crate::api::endpoint!(Updater);

impl<T: HttpTransport> Updater<'_, T> {
    pub async fn upload_package(&self, package: impl Into<Bytes>) -> Result<()> {
        let request = Call::post("/busybar/update").octet_stream(package);
        self.client.ok(request).await
    }

    pub async fn check(&self) -> Result<()> {
        self.client.ok(Call::post("/busybar/update/check")).await
    }

    pub async fn status(&self) -> Result<UpdateStatus> {
        self.client.json(Call::get("/busybar/update/status")).await
    }

    pub async fn changelog(&self, version: &str) -> Result<String> {
        let request = Call::get("/busybar/update/changelog").query("version", version);
        let response: ChangelogResponse = self.client.json(request).await?;
        Ok(response.changelog)
    }

    pub async fn install(&self, version: &str) -> Result<()> {
        let request = Call::post("/busybar/update/install").query("version", version);
        self.client.ok(request).await
    }

    pub async fn abort_download(&self) -> Result<()> {
        self.client
            .ok(Call::post("/busybar/update/abort_download"))
            .await
    }

    pub async fn autoupdate(&self) -> Result<AutoupdateSettings> {
        self.client
            .json(Call::get("/busybar/update/autoupdate"))
            .await
    }

    pub async fn set_autoupdate(&self, settings: &AutoupdateSettings) -> Result<()> {
        let request = Call::post("/busybar/update/autoupdate").json(settings)?;
        self.client.ok(request).await
    }
}
