use std::{collections::HashMap, ops::Add};

use serde::Deserialize;
use time::ext::NumericalDuration;

use crate::{
    products::evaporation::decode::{sort_coordinates_x, sort_coordinates_y},
    util::time::{format_date_iso, format_time_iso, timezone},
};

use super::RadolanResponse;

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum RadolanFormat {
    Default,
    SwmmRainfallData,
}

impl RadolanFormat {
    pub fn format_method(&self) -> fn(RadolanResponse, RadolanFormatConfig) -> String {
        match self {
            RadolanFormat::Default => format_default,
            RadolanFormat::SwmmRainfallData => format_swmm_rainfall_data,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RadolanFormatConfig {
    pub utc_to_berlin: bool,
    pub offset: i8,
}

pub fn format_default(evaporation: RadolanResponse, config: RadolanFormatConfig) -> String {
    let header = ["Name", "Date", "Time", "Value"];
    let mut str = header.join("\t");
    str.push('\n');

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
            let mut time = record.time;

            if config.utc_to_berlin {
                time = timezone::convert_utc_to_berlin(time);
            }
            if config.offset != 0 {
                time = time.add((config.offset as i64).minutes());
            }

            tmp.push(format!("{:0>4}_{:0>4}", coord.x, coord.y));
            tmp.push(format_date_iso(time.date()));
            tmp.push(format_time_iso(time.time()));
            tmp.push(format!("{:.2}", record.data[idx_table[idx]]));
            str.push_str(&tmp.join("\t"));
            str.push('\n');
            tmp.clear();
        }
    }
    str
}

pub fn format_swmm_rainfall_data(
    evaporation: RadolanResponse,
    config: RadolanFormatConfig,
) -> String {
    let header = ["Name", "Jahr", "Monat", "Tag", "Stunde", "Minute", "Wert"];
    let mut str = header.join("\t");
    str.push('\n');

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
            let mut time = record.time;

            if config.utc_to_berlin {
                time = timezone::convert_utc_to_berlin(time);
            }
            if config.offset != 0 {
                time = time.add((config.offset as i64).minutes());
            }

            tmp.push(format!("{:0>4}_{:0>4}", coord.x, coord.y));

            tmp.push(time.date().year().to_string());
            tmp.push(format!("{:0>2}", time.date().month() as u8));
            tmp.push(format!("{:0>2}", time.date().day()));
            tmp.push(format!("{:0>2}", time.time().hour()));
            tmp.push(format!("{:0>2}", time.time().minute()));

            tmp.push(format!("{:.2}", record.data[idx_table[idx]]));
            str.push_str(&tmp.join("\t"));
            str.push('\n');
            tmp.clear();
        }
    }
    str
}
