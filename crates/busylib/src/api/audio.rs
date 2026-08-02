//! Audio endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::audio::{AudioVolumeInfo, PlayAudio};
use crate::transport::HttpTransport;
use crate::types::volume::Volume;

crate::api::endpoint!(
    /// Audio controls
    Audio
);

impl<T: HttpTransport> Audio<'_, T> {
    /// Play audio file
    ///
    /// Plays an audio file from the assets directory.
    /// Supported formats include .snd files.
    pub async fn play(&self, audio: &PlayAudio) -> Result<()> {
        let request = Call::post("audio/play").json(audio)?;
        self.client.ok(request).await
    }

    /// Stop audio playback
    ///
    /// Stops any currently playing audio
    pub async fn stop(&self) -> Result<()> {
        self.client.ok(Call::delete("audio/play")).await
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
    /// Set audio volume value
    pub async fn set_volume(&self, volume: Volume) -> Result<()> {
        self.set(volume, false).await
    }

    /// Set audio volume silently
    ///
    /// Set audio volume value
    pub async fn set_volume_silently(&self, volume: Volume) -> Result<()> {
        self.set(volume, true).await
    }

    async fn set(&self, volume: Volume, silent: bool) -> Result<()> {
        let request = Call::post("audio/volume")
            .query("volume", volume.percent())
            .query("silent", u8::from(silent));
        self.client.ok(request).await
    }
}
