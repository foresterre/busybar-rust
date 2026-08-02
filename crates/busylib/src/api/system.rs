//! System endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::system::{
    LogDumpResponse, NetworkInterfaceInfo, Status, StatusDevice, StatusFirmware, StatusPower,
    StatusSystem, TransportType, VersionInfo,
};
use crate::transport::HttpTransport;
use crate::types::log_name::LogName;
use crate::types::try_into_value::TryIntoValue;

crate::api::endpoint!(
    /// System information and control
    System
);

impl<T: HttpTransport> System<'_, T> {
    /// Get API version information
    ///
    /// Retrieves API version
    pub async fn version(&self) -> Result<String> {
        let response: VersionInfo = self.client.json(Call::get("version")).await?;
        Ok(response.api_semver)
    }

    /// Get device network connection info
    ///
    /// Retrieves device transport type (usb/wifi)
    pub async fn transport(&self) -> Result<TransportType> {
        let response: NetworkInterfaceInfo = self.client.json(Call::get("transport")).await?;
        Ok(response.r#type)
    }

    /// Get device status
    pub async fn status(&self) -> Result<Status> {
        self.client.json(Call::get("status")).await
    }

    /// Get device info
    pub async fn device(&self) -> Result<StatusDevice> {
        self.client.json(Call::get("status/device")).await
    }

    /// Get firmware info
    pub async fn firmware(&self) -> Result<StatusFirmware> {
        self.client.json(Call::get("status/firmware")).await
    }

    /// Get system status
    pub async fn system_info(&self) -> Result<StatusSystem> {
        self.client.json(Call::get("status/system")).await
    }

    /// Get power status
    pub async fn power(&self) -> Result<StatusPower> {
        self.client.json(Call::get("status/power")).await
    }

    /// Dump captured log
    ///
    /// Snapshot the in-memory log buffer to a file (defaults to /ext/log.txt)
    pub async fn dump_log(&self) -> Result<String> {
        let response: LogDumpResponse = self.client.json(Call::post("log_dump")).await?;
        Ok(response.path)
    }

    /// Dump captured log
    ///
    /// Snapshot the in-memory log buffer to a file (defaults to /ext/log.txt)
    pub async fn dump_log_as(&self, filename: impl TryIntoValue<LogName>) -> Result<String> {
        let request = Call::post("log_dump").query("filename", filename.try_into_value()?);
        let response: LogDumpResponse = self.client.json(request).await?;
        Ok(response.path)
    }
}
