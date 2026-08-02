use crate::client::Call;
use crate::error::Result;
use crate::model::{
    DeviceInfo, DeviceStatus, FirmwareInfo, LogDumpResponse, NetworkInterfaceInfo, PowerInfo,
    SystemInfo, TransportType, VersionInfo,
};
use crate::transport::HttpTransport;
use crate::types::{LogName, TryIntoValue};

crate::api::endpoint!(System);

impl<T: HttpTransport> System<'_, T> {
    pub async fn version(&self) -> Result<String> {
        let response: VersionInfo = self.client.json(Call::get("/busybar/version")).await?;
        Ok(response.api_semver)
    }

    pub async fn transport(&self) -> Result<TransportType> {
        let response: NetworkInterfaceInfo =
            self.client.json(Call::get("/busybar/transport")).await?;
        Ok(response.r#type)
    }

    pub async fn status(&self) -> Result<DeviceStatus> {
        self.client.json(Call::get("/busybar/status")).await
    }

    pub async fn device(&self) -> Result<DeviceInfo> {
        self.client.json(Call::get("/busybar/status/device")).await
    }

    pub async fn firmware(&self) -> Result<FirmwareInfo> {
        self.client
            .json(Call::get("/busybar/status/firmware"))
            .await
    }

    pub async fn system_info(&self) -> Result<SystemInfo> {
        self.client.json(Call::get("/busybar/status/system")).await
    }

    pub async fn power(&self) -> Result<PowerInfo> {
        self.client.json(Call::get("/busybar/status/power")).await
    }

    pub async fn dump_log(&self) -> Result<String> {
        let response: LogDumpResponse = self.client.json(Call::post("/busybar/log_dump")).await?;
        Ok(response.path)
    }

    pub async fn dump_log_as(&self, filename: impl TryIntoValue<LogName>) -> Result<String> {
        let request = Call::post("/busybar/log_dump").query("filename", filename.try_into_value()?);
        let response: LogDumpResponse = self.client.json(request).await?;
        Ok(response.path)
    }
}
