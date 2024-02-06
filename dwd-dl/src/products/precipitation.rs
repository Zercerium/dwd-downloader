use std::vec;

use time::PrimitiveDateTime;

use crate::{
    dwd_source::{self, Common, CommonRequestData, DwdSource, Timespan},
    util::{
        interval::Interval,
        time::{format_date_american, format_time_colon, format_yyyymmddhhmm},
    },
};

use self::resolutions::*;

mod formats;
mod resolutions;

pub use formats::PrecipitationFormat;
pub use resolutions::PrecipitationResolution;

#[derive(Debug)]
pub struct PrecipitationCommonRequestData {
    pub common: CommonRequestData,
    pub station: String,
    pub resolution: PrecipitationResolution,
}

impl Common for PrecipitationCommonRequestData {
    fn common(&self) -> &CommonRequestData {
        &self.common
    }
}

#[derive(Debug, PartialEq)]
pub struct PrecipitationResponse {
    pub station: String,
    pub records: Vec<PrecipitationRecord>,
}

#[derive(Debug, PartialEq)]
pub struct PrecipitationRecord {
    pub timespan: Interval<PrimitiveDateTime>,
    pub rs: f32,
}

impl Timespan for PrecipitationRecord {
    fn timespan(&self) -> Interval<PrimitiveDateTime> {
        self.timespan
    }
}

impl dwd_source::Sources for PrecipitationCommonRequestData {
    type Record = PrecipitationRecord;

    fn sources(&self) -> Vec<Box<dyn DwdSource<Record = Self::Record, RequestData = Self>>> {
        match self.resolution {
            PrecipitationResolution::PrecipitationMin1 => vec![
                Box::new(precipitation_min1::PrecipitationMin1Historical),
                Box::new(precipitation_min1::PrecipitationMin1Recent),
                Box::new(precipitation_min1::PrecipitationMin1Now),
            ],
            PrecipitationResolution::PrecipitationMin5 => vec![
                Box::new(precipitation_min5::PrecipitationMin5Historical),
                Box::new(precipitation_min5::PrecipitationMin5Recent),
                Box::new(precipitation_min5::PrecipitationMin5Now),
            ],
            PrecipitationResolution::PrecipitationMin10 => vec![
                Box::new(precipitation_min10::Historical),
                Box::new(precipitation_min10::Recent),
                Box::new(precipitation_min10::Now),
            ],
            PrecipitationResolution::PrecipitationHourly => vec![
                Box::new(precipitation_hourly::Historical),
                Box::new(precipitation_hourly::Recent),
            ],
        }
    }
}

pub fn data_to_together(records: PrecipitationResponse) -> String {
    let mut string = String::new();
    let header = vec!["STATIONS_ID", "MESS_DATUM", "RS"];
    string.push_str(header.join("\t").as_str());
    string.push('\n');

    for record in records.records {
        let mut record_str = Vec::new();
        record_str.push(records.station.clone());
        let datetime_str = format_yyyymmddhhmm(record.timespan.start);
        record_str.push(datetime_str);
        record_str.push(format!("{:.2}", record.rs));

        string.push_str(&record_str.join("\t"));
        string.push('\n');
    }
    string
}

pub fn data_to_separated(records: PrecipitationResponse) -> String {
    let mut string = String::new();
    let header = vec!["Station", "Date", "Time", "Value"];
    string.push_str(header.join("\t").as_str());
    string.push('\n');

    for record in records.records {
        let mut record_str = Vec::new();
        record_str.push(records.station.clone());
        let date_str = format_date_american(record.timespan.start);
        record_str.push(date_str);
        let time_str = format_time_colon(record.timespan.start);
        record_str.push(time_str);
        record_str.push(format!("{:.2}", record.rs));

        string.push_str(&record_str.join("\t"));
        string.push('\n');
    }
    string
}

pub struct Product;

impl dwd_source::DwdProduct for Product {
    type Request = PrecipitationCommonRequestData;
    type Response = PrecipitationResponse;

    fn downloadx(&self, request: Self::Request) -> Self::Response {
        PrecipitationResponse {
            station: request.station.clone(),
            records: self.download(request),
        }
    }
}

#[cfg(test)]
mod test {
    use time::macros::datetime;

    use super::*;

    fn generate_common_data() -> PrecipitationResponse {
        PrecipitationResponse {
            station: "00001".to_string(),
            records: vec![
                PrecipitationRecord {
                    timespan: Interval::new(
                        datetime!(2022-01-10 20:00),
                        datetime!(2022-01-10 20:00),
                    )
                    .unwrap(),
                    rs: 10.0,
                },
                PrecipitationRecord {
                    timespan: Interval::new(
                        datetime!(2022-01-11 20:00),
                        datetime!(2022-01-11 20:00),
                    )
                    .unwrap(),
                    rs: 13.0,
                },
                PrecipitationRecord {
                    timespan: Interval::new(
                        datetime!(2022-01-12 20:00),
                        datetime!(2022-01-12 20:00),
                    )
                    .unwrap(),
                    rs: 9.0,
                },
            ],
        }
    }

    #[test]
    fn test_format_date_together() {
        let test_data = generate_common_data();
        let result = data_to_together(test_data);

        let assert = r#"STATIONS_ID\tMESS_DATUM\tRS
00001\t202201102000\t10.00
00001\t202201112000\t13.00
00001\t202201122000\t9.00
"#;
        let assert = assert.replace(r"\t", "\t");
        assert_eq!(result, assert);
    }

    #[test]
    fn test_format_date_separated() {
        let test_data = generate_common_data();
        let result = data_to_separated(test_data);

        let assert = r"Station\tDate\tTime\tValue
00001\t01/10/2022\t20:00\t10.00
00001\t01/11/2022\t20:00\t13.00
00001\t01/12/2022\t20:00\t9.00
";
        let assert = assert.replace(r"\t", "\t");
        assert_eq!(result, assert);
    }
}
