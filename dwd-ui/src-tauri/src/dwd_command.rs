use std::{fs, io::Write};

use dwd_dl::{
    dwd_source::{CommonRequestData, DwdProduct},
    products::{
        climate::{self, ClimateCommonRequestData, ClimateResolution},
        evaporation::{self, EvaporationRequest, EvaporationResolution},
        precipitation::{self, PrecipitationCommonRequestData, PrecipitationResolution},
        radolan::{self, formats::RadolanFormatConfig, RadolanRequest, RadolanResolution},
    },
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{
    utils::{ProgressBarState, ProgressBarStatus},
    Manager, Window,
};
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

        let start = self.start.replace(':', "-");
        let end = self.end.replace(':', "-");

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

impl TryInto<EvaporationRequest> for UniversalRequest {
    type Error = ();

    fn try_into(self) -> Result<EvaporationRequest, Self::Error> {
        match self.product {
            Product::Evaporation(o) => Ok(EvaporationRequest {
                common: CommonRequestData {
                    timespan: dwd_dl::util::interval::Interval {
                        start: time::PrimitiveDateTime::parse(&self.start, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                        end: time::PrimitiveDateTime::parse(&self.end, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                    },
                },
                coordinates: self
                    .coordinates
                    .lines()
                    .map(|s| s.parse().unwrap())
                    .collect(),
                resolution: o.resolution,
            }),
            _ => Err(()),
        }
    }
}

impl TryInto<RadolanRequest> for UniversalRequest {
    type Error = ();

    fn try_into(self) -> Result<RadolanRequest, Self::Error> {
        match self.product {
            Product::Radolan(o) => Ok(RadolanRequest {
                common: CommonRequestData {
                    timespan: dwd_dl::util::interval::Interval {
                        start: time::PrimitiveDateTime::parse(&self.start, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                        end: time::PrimitiveDateTime::parse(&self.end, &Iso8601::DEFAULT)
                            .map_err(|_| ())?,
                    },
                },
                coordinates: self
                    .coordinates
                    .lines()
                    .map(|s| s.parse().unwrap())
                    .collect(),
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
    pub format_config: RadolanFormatConfig,
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
        Product::Radolan(o) => {
            let request: RadolanRequest = request.clone().try_into().unwrap();
            let data = radolan::Product.downloadx(request);

            dbg!(&o.format_config);

            let formatter = o.format.format_method();
            let response = formatter(data, o.format_config);

            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(response.as_bytes()).unwrap();
        }
        Product::Evaporation(o) => {
            let request: EvaporationRequest = request.clone().try_into().unwrap();
            let data = evaporation::Product.downloadx(request);

            let formatter = o.format.format_method();
            let response = formatter(data);

            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(response.as_bytes()).unwrap();
        }
    }

    "success".to_string()
}

#[tauri::command]
pub fn dwd_filename_suggestion(request: UniversalRequest) -> String {
    request.filename()
}

#[derive(Serialize, Type, Debug, Clone)]
pub struct ProgressUpdate {
    pub progress: f32,
    pub message: Option<String>,
}

#[tauri::command]
pub async fn async_test(window: Window, success: bool) -> Result<String, String> {
    let (s, r) = crossbeam_channel::unbounded();

    let handle = std::thread::spawn(move || test_emit_progress_updates(s));

    std::thread::spawn({
        let window = window.clone();
        move || {
            while let Ok(update) = r.recv() {
                window
                    .set_progress_bar(ProgressBarState {
                        status: Some(ProgressBarStatus::Normal),
                        progress: Some((update.progress as u32).into()),
                        unity_uri: None,
                    })
                    .unwrap();
                window.emit("dwd-progress-update", update).unwrap();
            }
            println!("done");
        }
    });

    handle.join().unwrap();

    window
        .set_progress_bar(ProgressBarState {
            status: Some(ProgressBarStatus::None),
            progress: Some(0),
            unity_uri: None,
        })
        .unwrap();
    window
        .request_user_attention(Some(tauri::UserAttentionType::Informational))
        .unwrap();

    match success {
        true => Ok("success".to_string()),
        false => Err("failure".to_string()),
    }
}

fn test_emit_progress_updates(s: crossbeam_channel::Sender<ProgressUpdate>) {
    for i in 0..=100 {
        s.send(ProgressUpdate {
            progress: i as f32,
            message: None,
        })
        .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
