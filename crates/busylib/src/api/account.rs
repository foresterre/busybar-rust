//! Account endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::account::{AccountBackend, AccountInfo, AccountStatus, MqttStatus};
use crate::transport::HttpTransport;

crate::api::endpoint!(
    /// Account linking and MQTT status
    Account
);

impl<T: HttpTransport> Account<'_, T> {
    /// Get linked account info
    ///
    /// Retrieves linked account data
    pub async fn info(&self) -> Result<AccountInfo> {
        self.client.json(Call::get("/busybar/account/info")).await
    }

    /// Get MQTT status info
    ///
    /// Retrieves MQTT status
    pub async fn status(&self) -> Result<MqttStatus> {
        let response: AccountStatus = self
            .client
            .json(Call::get("/busybar/account/status"))
            .await?;
        Ok(response.status)
    }

    /// Get MQTT configuration
    ///
    /// Retrieves MQTT backend configuration
    pub async fn backend(&self) -> Result<AccountBackend> {
        self.client
            .json(Call::get("/busybar/account/backend"))
            .await
    }
}
