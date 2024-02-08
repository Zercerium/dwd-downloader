use time::PrimitiveDateTime;

use crate::{
    dwd_source::{Common, CommonRequestData, DwdProduct, DwdSource, Sources, Timespan},
    util::{interval::Interval, time::parse_yyyymmdd_into_date_time},
};

use self::resolutions::{
    climate_annual::{ClimateAnnualHistorical, ClimateAnnualRecent},
    climate_daily::{ClimateDailyHistorical, ClimateDailyRecent},
    climate_monthly::{ClimateMonthlyHistorical, ClimateMonthlyRecent},
};

mod formats;
mod resolutions;

pub use formats::ClimateFormat;
pub use resolutions::ClimateResolution;

#[derive(Debug, Clone)]
pub struct ClimateCommonRequestData {
    pub common: CommonRequestData,
    pub station: String,
    pub resolution: ClimateResolution,
}

impl Common for ClimateCommonRequestData {
    fn common(&self) -> &CommonRequestData {
        &self.common
    }
}

#[derive(Debug, Clone)]
pub struct ClimateRecord {
    timespan: Interval<PrimitiveDateTime>,
    data: String,
}

impl Timespan for ClimateRecord {
    fn timespan(&self) -> Interval<PrimitiveDateTime> {
        self.timespan
    }
}

impl Sources for ClimateCommonRequestData {
    type Record = ClimateRecord;

    fn sources(&self) -> Vec<Box<dyn DwdSource<Record = Self::Record, RequestData = Self>>> {
        match self.resolution {
            ClimateResolution::ClimateDaily => vec![
                Box::new(ClimateDailyHistorical),
                Box::new(ClimateDailyRecent),
            ],
            ClimateResolution::ClimateMonthly => vec![
                Box::new(ClimateMonthlyHistorical),
                Box::new(ClimateMonthlyRecent),
            ],
            ClimateResolution::ClimateAnnual => vec![
                Box::new(ClimateAnnualHistorical),
                Box::new(ClimateAnnualRecent),
            ],
        }
    }
}

impl ClimateResolution {
    pub fn header(&self) -> String {
        match self {
            ClimateResolution::ClimateDaily => "STATIONS_ID;MESS_DATUM;QN_3;  FX;  FM;QN_4; RSK;RSKF; SDK;SHK_TAG;  NM; VPM;  PM; TMK; UPM; TXK; TNK; TGK;eor",
            ClimateResolution::ClimateMonthly => "STATIONS_ID;MESS_DATUM_BEGINN;MESS_DATUM_ENDE;QN_4;MO_N;MO_TT;MO_TX;MO_TN;MO_FK;MX_TX;MX_FX;MX_TN;MO_SD_S;QN_6;MO_RR;MX_RS;eor",
            ClimateResolution::ClimateAnnual => "STATIONS_ID;MESS_DATUM_BEGINN;MESS_DATUM_ENDE;QN_4;JA_N;JA_TT;JA_TX;JA_TN;JA_FK;JA_SD_S;JA_MX_FX;JA_MX_TX;JA_MX_TN;QN_6;JA_RR;JA_MX_RS;eor",
        }.into()
    }
}

pub struct ClimateProduct;

impl DwdProduct for ClimateProduct {
    type Request = ClimateCommonRequestData;
    type Response = Vec<ClimateRecord>;

    fn downloadx(&self, request: Self::Request) -> Self::Response {
        self.download(&request)
    }
}

pub fn climate_data_to_string(
    records: Vec<ClimateRecord>,
    resolution: &ClimateResolution,
) -> String {
    let mut string = String::new();
    string.push_str(&resolution.header());
    string.push('\n');
    for record in records {
        string.push_str(&record.data);
        string.push('\n');
    }
    string
}

/// for STATIONS_ID;MESS_DATUM_BEGINN;MESS_DATUM_ENDE; ...
fn extract_timespan(s: &str) -> Interval<PrimitiveDateTime> {
    let times = s.split(';').skip(1).take(2).collect::<Vec<_>>();
    let start = parse_yyyymmdd_into_date_time(times[0]).unwrap();
    let end = parse_yyyymmdd_into_date_time(times[1]).unwrap();
    Interval::new(start, end).unwrap()
}
