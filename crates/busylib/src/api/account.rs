use crate::client::Call;
use crate::error::Result;
use crate::model::account::{AccountBackend, AccountInfo, AccountStatus, MqttStatus};
use crate::transport::HttpTransport;

crate::api::endpoint!(Account);

impl<T: HttpTransport> Account<'_, T> {
    pub async fn info(&self) -> Result<AccountInfo> {
        self.client.json(Call::get("/busybar/account/info")).await
    }

    pub async fn status(&self) -> Result<MqttStatus> {
        let response: AccountStatus = self
            .client
            .json(Call::get("/busybar/account/status"))
            .await?;
        Ok(response.status)
    }

    pub async fn backend(&self) -> Result<AccountBackend> {
        self.client
            .json(Call::get("/busybar/account/backend"))
            .await
    }
}
