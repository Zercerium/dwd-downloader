use std::{fs, io::Write};

use dwd_dl::{
    dwd_source::{CommonRequestData, DwdProduct},
    products::{
        climate::{self, ClimateCommonRequestData, ClimateResolution},
        evaporation::{self, EvaporationResolution},
        precipitation::{self, PrecipitationCommonRequestData, PrecipitationResolution},
        radolan::{self, RadolanResolution},
    },
};
use serde::Deserialize;
use specta::Type;
use time::format_description::well_known::Iso8601;

#[derive(Deserialize, Type, Debug, Clone)]
pub struct UniversalRequest {
    pub start: String,
    pub end: String,
    pub station: String,
    pub coordinates: String,
    pub product: Product,
    pub file_path: String,
}

impl UniversalRequest {
    pub fn filename(&self) -> String {
        let mut filename = String::from(&self.product.resolution_str());

        match &self.product {
            Product::Climate(_) | Product::Precipitation(_) => {
                filename.push_str(&format!("_{}", &self.station))
            }
            Product::Evaporation(_) | Product::Radolan(_) => (),
        }

        let start = self.start.replace(":", "-");
        let end = self.end.replace(":", "-");

        filename.push_str(&format!("_{}_{}", start, end));

        filename
    }
}

impl TryInto<ClimateCommonRequestData> for UniversalRequest {
    type Error = ();

    fn try_into(self) -> Result<ClimateCommonRequestData, Self::Error> {
        match self.product {
            Product::Climate(o) => Ok(ClimateCommonRequestData {
                common: CommonRequestData {
                    timespan: dwd_dl::util::interval::Interval {
                        start: time::PrimitiveDateTime::parse(&self.start, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                        end: time::PrimitiveDateTime::parse(&self.end, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                    },
                },
                station: self.station,
                resolution: o.resolution,
            }),
            _ => Err(()),
        }
    }
}

impl TryInto<PrecipitationCommonRequestData> for UniversalRequest {
    type Error = ();

    fn try_into(self) -> Result<PrecipitationCommonRequestData, Self::Error> {
        match self.product {
            Product::Precipitation(o) => Ok(PrecipitationCommonRequestData {
                common: CommonRequestData {
                    timespan: dwd_dl::util::interval::Interval {
                        start: time::PrimitiveDateTime::parse(&self.start, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                        end: time::PrimitiveDateTime::parse(&self.end, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                    },
                },
                station: self.station,
                resolution: o.resolution,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Type, Debug, Clone)]
pub enum Product {
    Climate(ClimateOptions),
    Precipitation(PrecipitationOptions),
    Radolan(RadolanOptions),
    Evaporation(EvaporationOptions),
}

impl Product {
    pub fn resolution_str(&self) -> String {
        match self {
            Product::Climate(o) => format!("{}", o.resolution),
            Product::Precipitation(o) => format!("{}", o.resolution),
            Product::Radolan(o) => format!("{}", o.resolution),
            Product::Evaporation(o) => format!("{}", o.resolution),
        }
    }
}

#[derive(Deserialize, Type, Debug, Clone, Copy)]
pub struct ClimateOptions {
    pub resolution: ClimateResolution,
    pub format: climate::ClimateFormat,
}

#[derive(Deserialize, Type, Debug, Clone, Copy)]
pub struct PrecipitationOptions {
    pub resolution: PrecipitationResolution,
    pub format: precipitation::PrecipitationFormat,
}

#[derive(Deserialize, Type, Debug, Clone, Copy)]
pub struct RadolanOptions {
    pub resolution: RadolanResolution,
    pub format: radolan::RadolanFormat,
}

#[derive(Deserialize, Type, Debug, Clone, Copy)]
pub struct EvaporationOptions {
    pub resolution: EvaporationResolution,
    pub format: evaporation::EvaporationFormat,
}

#[tauri::command]
pub fn dwd_request(request: UniversalRequest) -> String {
    let file = fs::File::create(&request.file_path).unwrap();

    match &request.product {
        Product::Climate(o) => {
            let request: ClimateCommonRequestData = request.clone().try_into().unwrap();
            let data = climate::ClimateProduct.downloadx(request);

            let formatter = o.format.format_method();
            let response = formatter(data, &o.resolution);

            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(response.as_bytes()).unwrap();
        }
        Product::Precipitation(o) => {
            let request: PrecipitationCommonRequestData = request.clone().try_into().unwrap();
            let data = precipitation::Product.downloadx(request);

            let formatter = o.format.format_method();
            let response = formatter(data);

            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(response.as_bytes()).unwrap();
        }
        // Product::Radolan(o) => {
        //     let data = radolan::RadolanProduct.download(request.station, o.resolution);
        //     let mut writer = std::io::BufWriter::new(file);
        //     writer.write_all(&data).unwrap();
        // }
        // Product::Evaporation(o) => {
        //     let data = evaporation::EvaporationProduct.download(request.station, o.resolution);
        //     let response = evaporation::evaporation_data_to_string(data, o.resolution);
        //     let mut writer = std::io::BufWriter::new(file);
        //     writer.write_all(response.as_bytes()).unwrap();
        // }
        _ => todo!(),
    }

    format!("{}", "success")
}

#[tauri::command]
pub fn dwd_filename_suggestion(request: UniversalRequest) -> String {
    request.filename()
}
