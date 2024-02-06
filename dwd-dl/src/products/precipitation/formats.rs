use serde::Deserialize;

use super::{data_to_separated, data_to_together, PrecipitationResponse};

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum PrecipitationFormat {
    DateTogether,
    DateSeparated,
}

impl PrecipitationFormat {
    pub fn format_method(&self) -> fn(PrecipitationResponse) -> String {
        match self {
            PrecipitationFormat::DateSeparated => data_to_separated,
            PrecipitationFormat::DateTogether => data_to_together,
        }
    }
}
