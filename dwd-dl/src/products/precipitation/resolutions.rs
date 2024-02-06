use serde::Deserialize;

pub mod precipitation_hourly;
pub mod precipitation_min1;
pub mod precipitation_min10;
pub mod precipitation_min5;

#[derive(Debug, Deserialize, Clone, Copy, strum_macros::Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum PrecipitationResolution {
    PrecipitationMin1,
    PrecipitationMin5,
    PrecipitationMin10,
    PrecipitationHourly,
}
