use crate::{
    base_url,
    dwd_source::{self, UrlTimeIntervall},
    products::climate::{ClimateCommonRequestData, ClimateRecord},
    util::{
        compression::zip,
        download::download_text,
        interval::Interval,
        regex::{extract_interval_d8_d8, links_in_text},
        time::parse_yyyymmdd_into_date_time,
    },
};

// https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/
// tageswerte_KL_00001_19370101_19860630_hist.zip
pub struct ClimateDailyHistorical;

impl dwd_source::DwdSource for ClimateDailyHistorical {
    type Record = ClimateRecord;
    type RequestData = ClimateCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/daily/kl/historical/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(
            r"tageswerte_KL_{}_\d{{8}}_\d{{8}}_hist.zip",
            request_data.station
        );

        let links = links_in_text(&html, &regex);
        links
            .iter()
            .map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: Some(extract_interval_d8_d8(link).unwrap().into()),
            })
            .inspect(|x| println!("{:?}", x))
            .collect::<Vec<_>>()
    }

    fn extract_data(&self, body: bytes::Bytes) -> Vec<Self::Record> {
        let bytes = zip::extract_file(body, "produkt");
        let data = String::from_utf8(bytes).unwrap();
        data.lines()
            .skip(1)
            .map(|line| {
                let time = parse_yyyymmdd_into_date_time(line.split(';').nth(1).unwrap()).unwrap();
                let timespan = Interval::new(time, time).unwrap();
                Self::Record {
                    data: line.to_string(),
                    timespan,
                }
            })
            .collect()
    }
}

// https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/recent/
// tageswerte_KL_00011_akt.zip
pub struct ClimateDailyRecent;

impl dwd_source::DwdSource for ClimateDailyRecent {
    type Record = ClimateRecord;
    type RequestData = ClimateCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/daily/kl/recent/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"tageswerte_KL_{}_akt.zip", request_data.station);

        let links = links_in_text(&html, &regex);
        links
            .iter()
            .map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: None,
            })
            .inspect(|x| println!("{:?}", x))
            .collect::<Vec<_>>()
    }

    fn extract_data(&self, body: bytes::Bytes) -> Vec<Self::Record> {
        let bytes = zip::extract_file(body, "produkt");
        let data = String::from_utf8(bytes).unwrap();
        data.lines()
            .skip(1)
            .map(|line| {
                let time = parse_yyyymmdd_into_date_time(line.split(';').nth(1).unwrap()).unwrap();
                let timespan = Interval::new(time, time).unwrap();
                Self::Record {
                    data: line.to_string(),
                    timespan,
                }
            })
            .collect()
    }
}
