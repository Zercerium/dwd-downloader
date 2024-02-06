use time::PrimitiveDateTime;

use crate::{
    base_url,
    dwd_source::{self, UrlTimeIntervall},
    products::precipitation::{PrecipitationCommonRequestData, PrecipitationRecord},
    util::{
        self,
        download::download_text,
        interval::Interval,
        regex::{extract_interval_d8_d8, links_in_text, year_links_in_text},
        time::parse_yyyymmddhhmm,
    },
};

pub struct PrecipitationMin1Historical;

impl dwd_source::DwdSource for PrecipitationMin1Historical {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/1_minute/precipitation/historical/",
            base_url()
        );
        let html = download_text(&url, None);
        let mut years = year_links_in_text(&html);
        years.sort_unstable();
        let ts = request_data.common.timespan;
        let mut urls = Vec::new();
        for year in ts.start.year()..=ts.end.year() {
            if years.binary_search(&year).is_ok() {
                urls.push(format!("{}{}/", url, year));
            }
        }

        //FIXME: dont recompile regex
        let mut links = Vec::new();
        for url in urls {
            let html = download_text(&url, None);
            let regex = format!(
                r"1minutenwerte_nieder_{}_\d{{8}}_\d{{8}}_hist.zip",
                request_data.station
            );
            let current_links = links_in_text(&html, &regex);

            links.extend(current_links.iter().map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: Some(extract_interval_d8_d8(link).unwrap().into()),
            }));
        }
        dbg!(&links);
        links
    }

    fn extract_data(&self, body: bytes::Bytes) -> Vec<Self::Record> {
        let bytes = util::compression::zip::extract_file(body, "produkt");
        let data = String::from_utf8(bytes).unwrap();
        data.lines()
            .skip(1)
            .map(|line| {
                let timespan = extract_timespan(&line);
                let rs = line.split(';').nth(4).unwrap().trim().parse().unwrap();
                Self::Record { rs, timespan }
            })
            .collect()
    }
}

pub struct PrecipitationMin1Recent;

impl dwd_source::DwdSource for PrecipitationMin1Recent {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/1_minute/precipitation/recent/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"1minutenwerte_nieder_{}_akt.zip", request_data.station);

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
                let rs = line.split(';').nth(3).unwrap().trim().parse().unwrap();
                Self::Record { rs, timespan }
            })
            .collect()
    }
}

pub struct PrecipitationMin1Now;

impl dwd_source::DwdSource for PrecipitationMin1Now {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/1_minute/precipitation/now/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"1minutenwerte_nieder_{}_now.zip", request_data.station);

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
                let rs = line.split(';').nth(3).unwrap().trim().parse().unwrap();
                Self::Record { rs, timespan }
            })
            .collect()
    }
}

/// for STATIONS_ID;MESS_DATUM_BEGINN;MESS_DATUM_ENDE; ...
fn extract_timespan(s: &str) -> Interval<PrimitiveDateTime> {
    let times = s.split(';').skip(1).take(2).collect::<Vec<_>>();
    let start = parse_yyyymmddhhmm(times[0]).unwrap();
    let end = parse_yyyymmddhhmm(times[1]).unwrap();
    Interval::new(start, end).unwrap()
}
