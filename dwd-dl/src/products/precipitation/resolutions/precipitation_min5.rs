use crate::{
    base_url,
    dwd_source::{self, UrlTimeIntervall},
    products::precipitation::{PrecipitationCommonRequestData, PrecipitationRecord},
    util::{
        self,
        download::download_text,
        file::File,
        interval::Interval,
        regex::{extract_interval_d8_d8, links_in_text, year_links_in_text},
        time::parse_yyyymmddhhmm,
    },
};

pub struct PrecipitationMin5Historical;

impl dwd_source::DwdSource for PrecipitationMin5Historical {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/5_minutes/precipitation/historical/",
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
        dbg!(&urls);

        //FIXME: dont recompile regex
        let mut links = Vec::new();
        for url in urls {
            let html = download_text(&url, None);
            let regex = format!(
                r"5minutenwerte_nieder_{}_\d{{8}}_\d{{8}}_hist.zip",
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

    fn extract_data(&self, _request_data: &Self::RequestData, file: File) -> Vec<Self::Record> {
        let bytes = util::compression::zip::extract_file(file.data, "produkt");
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

pub struct PrecipitationMin5Recent;

impl dwd_source::DwdSource for PrecipitationMin5Recent {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/5_minutes/precipitation/recent/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"5minutenwerte_nieder_{}_akt.zip", request_data.station);

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

    fn extract_data(&self, _request_data: &Self::RequestData, file: File) -> Vec<Self::Record> {
        let bytes = util::compression::zip::extract_file(file.data, "produkt");
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

pub struct PrecipitationMin5Now;

impl dwd_source::DwdSource for PrecipitationMin5Now {
    type Record = PrecipitationRecord;
    type RequestData = PrecipitationCommonRequestData;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}climate_environment/CDC/observations_germany/climate/5_minutes/precipitation/now/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = format!(r"5minutenwerte_nieder_{}_now.zip", request_data.station);

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

    fn extract_data(&self, _request_data: &Self::RequestData, file: File) -> Vec<Self::Record> {
        let bytes = util::compression::zip::extract_file(file.data, "produkt");
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
