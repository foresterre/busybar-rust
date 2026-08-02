//! Time endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::time::{TimestampInfo, TimezoneInfo, TimezoneListResponse};
use crate::transport::HttpTransport;
use crate::types::timestamp::Timestamp;
use crate::types::timezone_name::TimezoneName;
use crate::types::try_into_value::TryIntoValue;

crate::api::endpoint!(
    /// Time-related methods
    Time
);

impl<T: HttpTransport> Time<'_, T> {
    /// Get current timestamp with timezone
    ///
    /// Retrieves the current timestamp from RTC with timezone in ISO 8601 format
    pub async fn now(&self) -> Result<Timestamp> {
        let response: TimestampInfo = self.client.json(Call::get("time")).await?;
        Ok(response.timestamp)
    }

    /// Set current timestamp
    ///
    /// Sets the RTC timestamp in ISO 8601 format. Time zone qualifier (e.g. Z of UTC or +hh:mm for
    /// local time) is required.
    pub async fn set_now(&self, timestamp: impl TryIntoValue<Timestamp>) -> Result<()> {
        let request = Call::post("time/timestamp").query("timestamp", timestamp.try_into_value()?);
        self.client.ok(request).await
    }

    /// Get timezone
    ///
    /// Get current timezone name
    pub async fn timezone(&self) -> Result<TimezoneInfo> {
        self.client.json(Call::get("time/timezone")).await
    }

    /// Set timezone
    ///
    /// Sets the timezone name. Use /api/time/tzlist to get available names list.
    pub async fn set_timezone(&self, timezone: impl TryIntoValue<TimezoneName>) -> Result<()> {
        let request = Call::post("time/timezone").query("timezone", timezone.try_into_value()?);
        self.client.ok(request).await
    }

    /// Get list of supported time zones
    ///
    /// Retrieves the list of time zones accepted by /api/time/timezone
    pub async fn timezones(&self) -> Result<Vec<TimezoneInfo>> {
        let response: TimezoneListResponse = self.client.json(Call::get("time/tzlist")).await?;
        Ok(response.list)
    }
}
