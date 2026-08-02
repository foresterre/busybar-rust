//! Settings endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::settings::{
    AudioVolumeInfo, DisplayBrightnessInfo, HttpAccess, HttpAccessInfo, NameInfo,
};
use crate::transport::HttpTransport;
use crate::types::brightness::Brightness;
use crate::types::device_name::DeviceName;
use crate::types::try_into_value::TryIntoValue;
use crate::types::volume::Volume;

crate::api::endpoint!(
    /// Device settings, including audio volume and display brightness
    Settings
);

impl<T: HttpTransport> Settings<'_, T> {
    /// Get HTTP API access over Wi-Fi configuration
    pub async fn access(&self) -> Result<HttpAccessInfo> {
        self.client.json(Call::get("access")).await
    }

    /// Set HTTP API access over Wi-Fi configuration
    pub async fn set_access(&self, access: &HttpAccess) -> Result<()> {
        let request = Call::post("access")
            .query("mode", access.mode().as_str())
            .maybe_query("key", access.key().map(|key| key.as_str()));
        self.client.ok(request).await
    }

    /// Get current device name
    pub async fn name(&self) -> Result<DeviceName> {
        let response: NameInfo = self.client.json(Call::get("name")).await?;
        Ok(response.name)
    }

    /// Set new device name
    pub async fn set_name(&self, name: impl TryIntoValue<DeviceName>) -> Result<()> {
        let body = NameInfo {
            name: name.try_into_value()?,
        };
        let request = Call::post("name").json(&body)?;
        self.client.ok(request).await
    }

    /// Get audio volume
    ///
    /// Get audio volume value
    pub async fn volume(&self) -> Result<Volume> {
        let response: AudioVolumeInfo = self.client.json(Call::get("audio/volume")).await?;
        Ok(response.volume)
    }

    /// Set audio volume
    ///
    /// Set audio volume value. When `silent` is set, the volume change sound is not played.
    pub async fn set_volume(&self, volume: Volume, silent: bool) -> Result<()> {
        let request = Call::post("audio/volume")
            .query("volume", volume.percent())
            .query("silent", u8::from(silent));
        self.client.ok(request).await
    }

    /// Get display brightness
    ///
    /// Get brightness value for displays
    pub async fn brightness(&self) -> Result<Brightness> {
        let response: DisplayBrightnessInfo =
            self.client.json(Call::get("display/brightness")).await?;
        Ok(response.value)
    }

    /// Set display brightness
    ///
    /// Set brightness for one or both displays
    pub async fn set_brightness(&self, brightness: Brightness) -> Result<()> {
        let request = Call::post("display/brightness").query("value", brightness);
        self.client.ok(request).await
    }
}
