use serde::Deserialize;

pub mod radolan_daily;
pub mod radolan_hourly;
pub mod radolan_hourly_reproc2017;
pub mod radolan_min5;
pub mod radolan_min5_reproc2017;

#[derive(Debug, Deserialize, Clone, Copy, strum_macros::Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum RadolanResolution {
    RadolanDaily,
    RadolanHourly,
    RadolanHourlyReproc2017,
    RadolanMin5,
    RadolanMin5Reproc2017,
}
