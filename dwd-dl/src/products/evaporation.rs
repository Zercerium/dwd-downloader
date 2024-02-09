use time::{Date, PrimitiveDateTime};

use crate::{
    dwd_source::{Common, CommonRequestData, DwdProduct, DwdSource, Sources, Timespan},
    util::{interval::Interval, point::Point},
};

pub mod decode;
pub mod formats;
mod resolutions;

pub use formats::EvaporationFormat;
pub use resolutions::EvaporationResolution;

#[derive(Debug, Clone)]
pub struct EvaporationRequest {
    pub common: CommonRequestData,
    pub coordinates: Vec<Point<usize>>,
    pub resolution: EvaporationResolution,
}

impl Common for EvaporationRequest {
    fn common(&self) -> &CommonRequestData {
        &self.common
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub time: Date,
    pub data: Vec<f32>,
}

impl Timespan for Record {
    fn timespan(&self) -> Interval<PrimitiveDateTime> {
        Interval::new(self.time, self.time).unwrap().into()
    }
}

impl Sources for EvaporationRequest {
    type Record = Record;

    fn sources(&self) -> Vec<Box<dyn DwdSource<Record = Self::Record, RequestData = Self>>> {
        match self.resolution {
            EvaporationResolution::EvaporationDailyP => {
                vec![Box::new(resolutions::evaporation_daily_p::DailyP)]
            }
            EvaporationResolution::EvaporationDailyR => {
                vec![Box::new(resolutions::evaporation_daily_r::DailyR)]
            }
            EvaporationResolution::EvaporationMonthlyP => {
                vec![Box::new(resolutions::evaporation_monthly_p::MonthlyP)]
            }
            EvaporationResolution::EvaporationMonthlyR => {
                vec![Box::new(resolutions::evaporation_monthly_r::MonthlyR)]
            }
        }
    }
}

#[derive(Debug)]
pub struct EvaporationResponse {
    pub coordinates: Vec<Point<usize>>,
    pub records: Vec<Record>,
}

pub struct Product;

impl DwdProduct for Product {
    type Request = EvaporationRequest;
    type Response = EvaporationResponse;

    fn downloadx(&self, request: Self::Request) -> Self::Response {
        let records = self.download(&request);
        EvaporationResponse {
            coordinates: request.coordinates,
            records,
        }
    }
}
