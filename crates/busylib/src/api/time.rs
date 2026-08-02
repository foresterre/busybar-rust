use crate::client::Call;
use crate::error::Result;
use crate::model::{TimestampInfo, TimezoneInfo, TimezoneListResponse};
use crate::transport::HttpTransport;
use crate::types::{Timestamp, TimezoneName, TryIntoValue};

crate::api::endpoint!(Time);

impl<T: HttpTransport> Time<'_, T> {
    pub async fn now(&self) -> Result<Timestamp> {
        let response: TimestampInfo = self.client.json(Call::get("/busybar/time")).await?;
        Ok(response.timestamp)
    }

    pub async fn set_now(&self, timestamp: impl TryIntoValue<Timestamp>) -> Result<()> {
        let request =
            Call::post("/busybar/time/timestamp").query("timestamp", timestamp.try_into_value()?);
        self.client.ok(request).await
    }

    pub async fn timezone(&self) -> Result<TimezoneInfo> {
        self.client.json(Call::get("/busybar/time/timezone")).await
    }

    pub async fn set_timezone(&self, timezone: impl TryIntoValue<TimezoneName>) -> Result<()> {
        let request =
            Call::post("/busybar/time/timezone").query("timezone", timezone.try_into_value()?);
        self.client.ok(request).await
    }

    pub async fn timezones(&self) -> Result<Vec<TimezoneInfo>> {
        let response: TimezoneListResponse =
            self.client.json(Call::get("/busybar/time/tzlist")).await?;
        Ok(response.list)
    }
}
