use serde::Deserialize;

pub mod evaporation_daily_p;
pub mod evaporation_daily_r;
pub mod evaporation_monthly_p;
pub mod evaporation_monthly_r;

#[derive(Debug, Deserialize, Clone, Copy, strum_macros::Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum EvaporationResolution {
    EvaporationDailyP,
    EvaporationDailyR,
    EvaporationMonthlyP,
    EvaporationMonthlyR,
}
