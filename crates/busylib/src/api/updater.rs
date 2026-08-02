//! Updater endpoints

use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::updater::{AutoupdateSettings, ChangelogResponse, UpdateStatus};
use crate::transport::HttpTransport;

crate::api::endpoint!(
    /// Firmware update control
    Updater
);

impl<T: HttpTransport> Updater<'_, T> {
    /// Update firmware
    ///
    /// Uploads a firmware update package (TAR file) and initiates the update process.
    pub async fn upload_package(&self, package: impl Into<Bytes>) -> Result<()> {
        let request = Call::post("update").octet_stream(package);
        self.client.ok(request).await
    }

    /// Start firmware update check
    ///
    /// Initiates an asynchronous check for available firmware updates.
    pub async fn check(&self) -> Result<()> {
        self.client.ok(Call::post("update/check")).await
    }

    /// Get firmware update status
    ///
    /// Returns current update and check status including progress information.
    pub async fn status(&self) -> Result<UpdateStatus> {
        self.client.json(Call::get("update/status")).await
    }

    /// Get update changelog
    ///
    /// Returns the changelog for a specific firmware version.
    pub async fn changelog(&self, version: &str) -> Result<String> {
        let request = Call::get("update/changelog").query("version", version);
        let response: ChangelogResponse = self.client.json(request).await?;
        Ok(response.changelog)
    }

    /// Install firmware update
    ///
    /// Starts asynchronous firmware installation from a remote URL.
    /// The update process (download, SHA verification, unpack, prepare, reboot) runs in the
    /// background.
    /// Use /api/update/status to monitor progress.
    pub async fn install(&self, version: &str) -> Result<()> {
        let request = Call::post("update/install").query("version", version);
        self.client.ok(request).await
    }

    /// Abort ongoing firmware download
    ///
    /// Signals the updater to abort an ongoing download operation.
    pub async fn abort_download(&self) -> Result<()> {
        self.client.ok(Call::post("update/abort_download")).await
    }

    /// Get autoupdate settings
    ///
    /// Returns current autoupdate configuration
    pub async fn autoupdate(&self) -> Result<AutoupdateSettings> {
        self.client.json(Call::get("update/autoupdate")).await
    }

    /// Set autoupdate settings
    ///
    /// Updates autoupdate configuration. All fields are optional - only provided fields are
    /// updated.
    pub async fn set_autoupdate(&self, settings: &AutoupdateSettings) -> Result<()> {
        let request = Call::post("update/autoupdate").json(settings)?;
        self.client.ok(request).await
    }
}
