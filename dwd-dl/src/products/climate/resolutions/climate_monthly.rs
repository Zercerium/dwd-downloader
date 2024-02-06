use crate::{
    base_url,
    dwd_source::{self, UrlTimeIntervall},
    products::climate::{extract_timespan, ClimateCommonRequestData, ClimateRecord},
    util::{
        compression::zip,
        download::download_text,
        regex::{extract_interval_d8_d8, links_in_text},
    },
};

// https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/monthly/kl/historical/
// monatswerte_KL_00001_19310101_19860630_hist.zip
pub struct ClimateMonthlyHistorical;

impl dwd_source::DwdSource for ClimateMonthlyHistorical {
    type Record = ClimateRecord;
    type RequestData = ClimateCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/monthly/kl/historical/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(
            r"monatswerte_KL_{}_\d{{8}}_\d{{8}}_hist.zip",
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
            .map(|line| Self::Record {
                data: line.to_string(),
                timespan: extract_timespan(line),
            })
            .collect()
    }
}

// https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/monthly/kl/recent/
// monatswerte_KL_00044_akt.zip
pub struct ClimateMonthlyRecent;

impl dwd_source::DwdSource for ClimateMonthlyRecent {
    type Record = ClimateRecord;
    type RequestData = ClimateCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/monthly/kl/recent/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"monatswerte_KL_{}_akt.zip", request_data.station);

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
            .map(|line| Self::Record {
                data: line.to_string(),
                timespan: extract_timespan(line),
            })
            .collect()
    }
}
