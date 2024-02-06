use serde::Deserialize;

use super::{climate_data_to_string, ClimateRecord, ClimateResolution};

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ClimateFormat {
    Standard,
}

impl ClimateFormat {
    pub fn format_method(&self) -> fn(Vec<ClimateRecord>, &ClimateResolution) -> String {
        match self {
            ClimateFormat::Standard => climate_data_to_string,
        }
    }
}
