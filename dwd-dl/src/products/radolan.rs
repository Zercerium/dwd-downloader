use time::PrimitiveDateTime;

use crate::{
    dwd_source::{Common, CommonRequestData, DwdProduct, DwdSource, Sources, Timespan},
    util::{interval::Interval, point::Point},
};

mod decode;
pub mod formats;
mod resolutions;

pub use formats::RadolanFormat;
pub use resolutions::RadolanResolution;

#[derive(Debug, Clone)]
pub struct RadolanRequest {
    pub common: CommonRequestData,
    pub coordinates: Vec<Point<u16>>,
    pub resolution: RadolanResolution,
}

impl Common for RadolanRequest {
    fn common(&self) -> &CommonRequestData {
        &self.common
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub time: PrimitiveDateTime,
    pub data: Vec<f32>,
}

impl Timespan for Record {
    fn timespan(&self) -> Interval<PrimitiveDateTime> {
        Interval::new(self.time, self.time).unwrap()
    }
}

impl Sources for RadolanRequest {
    type Record = Record;

    fn sources(&self) -> Vec<Box<dyn DwdSource<Record = Self::Record, RequestData = Self>>> {
        match self.resolution {
            RadolanResolution::RadolanDaily => vec![
                Box::new(resolutions::radolan_daily::Historical),
                Box::new(resolutions::radolan_daily::Recent),
            ],
            RadolanResolution::RadolanHourly => vec![
                Box::new(resolutions::radolan_hourly::Historical),
                Box::new(resolutions::radolan_hourly::Recent),
            ],
            RadolanResolution::RadolanHourlyReproc2017 => vec![Box::new(
                resolutions::radolan_hourly_reproc2017::Reproc2017_002,
            )],
            RadolanResolution::RadolanMin5 => vec![Box::new(resolutions::radolan_min5::Recent)],
            RadolanResolution::RadolanMin5Reproc2017 => vec![Box::new(
                resolutions::radolan_min5_reproc2017::Reproc2017_002,
            )],
        }
    }
}

#[derive(Debug)]
pub struct RadolanResponse {
    pub coordinates: Vec<Point<u16>>,
    pub records: Vec<Record>,
}

pub struct Product;

impl DwdProduct for Product {
    type Request = RadolanRequest;
    type Response = RadolanResponse;

    fn downloadx(&self, request: Self::Request) -> Self::Response {
        let records = self.download(&request);
        RadolanResponse {
            coordinates: request.coordinates,
            records,
        }
    }
}
