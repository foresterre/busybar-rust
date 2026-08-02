//! Api endpoints
//!
//! Unlike the busy bar OpenAPI spec (23.0.0), it's grouped per path,
//! so `/audio` and `/display` are their own structs, instead of sitting under
//! `/assets`.

mod account;
mod assets;
mod audio;
mod ble;
mod busy;
mod display;
mod input;
mod settings;
mod smart_home;
mod storage;
mod system;
mod time;
mod updater;
mod wifi;

pub use account::Account;
pub use assets::Assets;
pub use audio::Audio;
pub use ble::Ble;
pub use busy::Busy;
pub use display::Display;
pub use input::Input;
pub use settings::Settings;
pub use smart_home::SmartHome;
pub use storage::Storage;
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
