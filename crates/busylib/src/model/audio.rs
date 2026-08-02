use serde::{Deserialize, Serialize};

use crate::types::{AppName, AssetPath, InvalidValue, StockPath, TryIntoValue, Volume};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayAudio {
    /// Application ID for organizing assets
    pub application_name: AppName,
    #[serde(flatten)]
    pub source: AudioSource,
}

impl PlayAudio {
    pub fn asset(
        application_name: impl TryIntoValue<AppName>,
        path: impl TryIntoValue<AssetPath>,
    ) -> Result<Self, InvalidValue> {
        Ok(Self {
            application_name: application_name.try_into_value()?,
            source: AudioSource::Asset {
                path: path.try_into_value()?,
            },
        })
    }

    pub fn stock(
        application_name: impl TryIntoValue<AppName>,
        stock_path: impl TryIntoValue<StockPath>,
    ) -> Result<Self, InvalidValue> {
        Ok(Self {
            application_name: application_name.try_into_value()?,
            source: AudioSource::Stock {
                stock_path: stock_path.try_into_value()?,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AudioSource {
    Asset {
        /// Path to audio file within app's assets directory
        path: AssetPath,
    },
    Stock {
        /// Stock audio file name
        stock_path: StockPath,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct AudioVolumeInfo {
    /// Audio volume value (0-100)
    pub volume: Volume,
}
