use std::collections::HashMap;

use serde::Deserialize;

use crate::util::time::{format_date_american, format_date_iso};

use super::{
    decode::{sort_coordinates_x, sort_coordinates_y},
    EvaporationResponse,
};

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum EvaporationFormat {
    Default,
    SwmmRainfallData,
}

impl EvaporationFormat {
    pub fn format_method(&self) -> fn(EvaporationResponse) -> String {
        match self {
            EvaporationFormat::Default => format_default,
            EvaporationFormat::SwmmRainfallData => format_swmm_rainfall_data,
        }
    }
}

pub fn format_default(evaporation: EvaporationResponse) -> String {
    let header = ["x_y", "Date", "Value"];
    let mut str = header.join("\t");
    str.push_str("\n");

    let coords = sort_coordinates_y(&evaporation.coordinates);
    let coords_idx = coords
        .iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();
    let coords_x = sort_coordinates_x(&evaporation.coordinates);
    let idx_table = coords_x
        .iter()
        .map(|p| *coords_idx.get(p).unwrap())
        .collect::<Vec<_>>();

    let mut tmp = Vec::new();
    for (idx, coord) in coords_x.iter().enumerate() {
        for record in &evaporation.records {
            tmp.push(format!("{:0>3}_{:0>3}", coord.x, coord.y));
            tmp.push(format_date_iso(record.time));
            tmp.push(format!("{:.1}", record.data[idx_table[idx]]));
            str.push_str(&tmp.join("\t"));
            str.push_str("\n");
            tmp.clear();
        }
    }
    str
}

// TODO check on start if only one Point is given
pub fn format_swmm_rainfall_data(evaporation: EvaporationResponse) -> String {
    let header = ["MM/DD/YYYY", "hh:mm", "Value"];
    let mut str = header.join("\t");
    str.push_str("\n");

    let time = "00:00";
    let mut tmp = Vec::new();
    for record in evaporation.records {
        tmp.push(format_date_american(record.time.midnight()));
        tmp.push(time.into());
        tmp.push(format!("{:.1}", record.data[0]));
        str.push_str(&tmp.join("\t"));
        str.push_str("\n");
        tmp.clear();
    }
    str
}
