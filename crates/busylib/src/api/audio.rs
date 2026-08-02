use crate::client::Call;
use crate::error::Result;
use crate::model::audio::{AudioVolumeInfo, PlayAudio};
use crate::transport::HttpTransport;
use crate::types::Volume;

crate::api::endpoint!(Audio);

impl<T: HttpTransport> Audio<'_, T> {
    pub async fn play(&self, audio: &PlayAudio) -> Result<()> {
        let request = Call::post("/busybar/audio/play").json(audio)?;
        self.client.ok(request).await
    }

    pub async fn stop(&self) -> Result<()> {
        self.client.ok(Call::delete("/busybar/audio/play")).await
    }

    pub async fn volume(&self) -> Result<Volume> {
        let response: AudioVolumeInfo =
            self.client.json(Call::get("/busybar/audio/volume")).await?;
        Ok(response.volume)
    }

    pub async fn set_volume(&self, volume: Volume) -> Result<()> {
        self.set(volume, false).await
    }

    pub async fn set_volume_silently(&self, volume: Volume) -> Result<()> {
        self.set(volume, true).await
    }

    async fn set(&self, volume: Volume, silent: bool) -> Result<()> {
        let request = Call::post("/busybar/audio/volume")
            .query("volume", volume.percent())
            .query("silent", u8::from(silent));
        self.client.ok(request).await
    }
}
