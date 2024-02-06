use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum EvaporationFormat {
    Default,
    SwmmRainfallData,
}
