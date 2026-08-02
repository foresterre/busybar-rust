//! BUSY timer endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::busy::{BusyProfile, BusyProfileSlot, BusySnapshot};
use crate::transport::HttpTransport;

crate::api::endpoint!(
    /// BUSY timer control
    Busy
);

impl<T: HttpTransport> Busy<'_, T> {
    /// Get BUSY timer snapshot
    ///
    /// Gets the current state of the BUSY timer in snapshot form
    pub async fn snapshot(&self) -> Result<BusySnapshot> {
        self.client.json(Call::get("busy/snapshot")).await
    }

    /// Set BUSY timer snapshot
    ///
    /// Run the timer starting from the given snapshot
    pub async fn set_snapshot(&self, snapshot: &BusySnapshot) -> Result<()> {
        let request = Call::put("busy/snapshot").json(snapshot)?;
        self.client.ok(request).await
    }

    /// Get BUSY timer profile
    ///
    /// Gets the BUSY timer profile under specified slot
    pub async fn profile(&self, slot: BusyProfileSlot) -> Result<BusyProfile> {
        self.client.json(Call::get(profile_path(slot))).await
    }

    /// Set BUSY timer profile
    ///
    /// Sets the BUSY timer profile under specified slot
    pub async fn set_profile(&self, slot: BusyProfileSlot, profile: &BusyProfile) -> Result<()> {
        let request = Call::put(profile_path(slot)).json(profile)?;
        self.client.ok(request).await
    }
}

fn profile_path(slot: BusyProfileSlot) -> &'static str {
    match slot {
        BusyProfileSlot::Busy => "busy/profiles/busy",
        BusyProfileSlot::Custom => "busy/profiles/custom",
    }
}
