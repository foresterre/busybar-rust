//! Api endpoints
//!
//! Grouped by the tags of the busy bar OpenAPI spec rather than by path, so `/audio/play`
//! and `/display/draw` sit under [`Assets`], `/audio/volume` and `/display/brightness`
//! under [`Settings`], and `/screen` under [`Streaming`].

mod account;
mod assets;
mod ble;
mod busy;
mod input;
mod settings;
mod smart_home;
mod storage;
mod streaming;
mod system;
mod time;
mod updater;
mod wifi;

pub use account::Account;
pub use assets::Assets;
pub use ble::Ble;
pub use busy::Busy;
pub use input::Input;
pub use settings::Settings;
pub use smart_home::SmartHome;
pub use storage::Storage;
pub use streaming::Streaming;
pub use system::System;
pub use time::Time;
pub use updater::Updater;
pub use wifi::Wifi;

macro_rules! endpoint {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        pub struct $name<'a, T> {
            client: &'a crate::client::Client<T>,
        }

        impl<'a, T> $name<'a, T> {
            pub(crate) fn new(client: &'a crate::client::Client<T>) -> Self {
                Self { client }
            }
        }

        impl<T> std::fmt::Debug for $name<'_, T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(concat!(stringify!($name), " endpoints"))
            }
        }
    };
}

pub(crate) use endpoint;
