use crate::client::Call;
use crate::error::Result;
use crate::model::busy::{BusyProfile, BusyProfileSlot, BusySnapshot};
use crate::transport::HttpTransport;

crate::api::endpoint!(Busy);

impl<T: HttpTransport> Busy<'_, T> {
    pub async fn snapshot(&self) -> Result<BusySnapshot> {
        self.client.json(Call::get("/busybar/busy/snapshot")).await
    }

    pub async fn set_snapshot(&self, snapshot: &BusySnapshot) -> Result<()> {
        let request = Call::put("/busybar/busy/snapshot").json(snapshot)?;
        self.client.ok(request).await
    }

    pub async fn profile(&self, slot: BusyProfileSlot) -> Result<BusyProfile> {
        self.client.json(Call::get(profile_path(slot))).await
    }

    pub async fn set_profile(&self, slot: BusyProfileSlot, profile: &BusyProfile) -> Result<()> {
        let request = Call::put(profile_path(slot)).json(profile)?;
        self.client.ok(request).await
    }
}

fn profile_path(slot: BusyProfileSlot) -> &'static str {
    match slot {
        BusyProfileSlot::Busy => "/busybar/busy/profiles/busy",
        BusyProfileSlot::Custom => "/busybar/busy/profiles/custom",
    }
}
