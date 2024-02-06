use crate::{
    base_url,
    dwd_source::{self, UrlTimeIntervall},
    products::precipitation::{PrecipitationCommonRequestData, PrecipitationRecord},
    util::{
        self,
        download::download_text,
        interval::Interval,
        regex::{extract_interval_d8_d8, links_in_text},
        time::parse_yyyymmddhhmm,
    },
};

pub struct Historical;

impl dwd_source::DwdSource for Historical {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/10_minutes/precipitation/historical/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(
            r"10minutenwerte_nieder_{}_\d{{8}}_\d{{8}}_hist.zip",
            request_data.station
        );
        let links = links_in_text(&html, &regex);

        links
            .iter()
            .map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: Some(extract_interval_d8_d8(link).unwrap().into()),
            })
            .collect::<Vec<_>>()
    }

    fn extract_data(&self, body: bytes::Bytes) -> Vec<Self::Record> {
        let bytes = util::compression::zip::extract_file(body, "produkt");
        let data = String::from_utf8(bytes).unwrap();
        data.lines()
            .skip(1)
            .map(|line| {
                let time = parse_yyyymmddhhmm(line.split(';').nth(1).unwrap()).unwrap();
                let timespan = Interval::new(time, time).unwrap();
                let rs = line.split(';').nth(4).unwrap().trim().parse().unwrap();
                Self::Record { rs, timespan }
            })
            .collect()
    }
}

pub struct Recent;

impl dwd_source::DwdSource for Recent {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/10_minutes/precipitation/recent/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"10minutenwerte_nieder_{}_akt.zip", request_data.station);

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
        let bytes = util::compression::zip::extract_file(body, "produkt");
        let data = String::from_utf8(bytes).unwrap();
        data.lines()
            .skip(1)
            .map(|line| {
                let time = parse_yyyymmddhhmm(line.split(';').nth(1).unwrap()).unwrap();
                let timespan = Interval::new(time, time).unwrap();
                let rs = line.split(';').nth(5).unwrap().trim().parse().unwrap();
                Self::Record { rs, timespan }
            })
            .collect()
    }
}

pub struct Now;

impl dwd_source::DwdSource for Now {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/10_minutes/precipitation/now/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"10minutenwerte_nieder_{}_now.zip", request_data.station);

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
        let bytes = util::compression::zip::extract_file(body, "produkt");
        let data = String::from_utf8(bytes).unwrap();
        data.lines()
            .skip(1)
            .map(|line| {
                let time = parse_yyyymmddhhmm(line.split(';').nth(1).unwrap()).unwrap();
                let timespan = Interval::new(time, time).unwrap();
                let rs = line.split(';').nth(5).unwrap().trim().parse().unwrap();
                Self::Record { rs, timespan }
            })
            .collect()
    }
}
