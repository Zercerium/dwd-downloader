use serde::Deserialize;

pub mod climate_annual;
pub mod climate_daily;
pub mod climate_monthly;

#[derive(Debug, Deserialize, Clone, Copy, strum_macros::Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ClimateResolution {
    ClimateDaily,
    ClimateMonthly,
    ClimateAnnual,
}
