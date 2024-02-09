use time::{util::days_in_year_month, Date, PrimitiveDateTime};

use crate::{
    base_url,
    dwd_source::{self, Common, UrlTimeIntervall},
    products::radolan::{decode::RadolanFile, RadolanRequest, Record},
    util::{
        compression::universal::MultiLayerFolder,
        download::download_text,
        file::File,
        interval::Interval,
        regex::{extract_d10, extract_d6, links_in_text, year_links_in_text},
        time::{parse_yyyymm, parse_yyyymmddhhmm},
    },
};

pub struct Historical;

impl dwd_source::DwdSource for Historical {
    type Record = Record;
    type RequestData = RadolanRequest;

    fn urls(&self, request: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}/climate_environment/CDC/grids_germany/hourly/radolan/historical/bin/",
            base_url()
        );
        let html = download_text(&url, None);
        let mut years = year_links_in_text(&html);
        years.sort_unstable();
        let ts = request.common.timespan;
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
            let regex = r"RW-?\d{6}.tar(.gz)?";
            let current_links = links_in_text(&html, &regex);

            links.extend(current_links.iter().map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: Some({
                    let date = extract_d6(link).unwrap();
                    let date = parse_yyyymm(&date).unwrap();
                    year_month_to_interval(date)
                }),
            }));
        }
        dbg!(&links);
        links
    }

    fn extract_data(&self, request_data: &Self::RequestData, file: File) -> Vec<Self::Record> {
        let filter0 = |_: &str| true;
        let ts = request_data.common().timespan.clone();
        let filter1 = move |s: &str| {
            let date = extract_d10(s).unwrap();
            let date = format!("20{}", date);
            let date = parse_yyyymmddhhmm(&date).unwrap();
            ts.contains(&date)
        };
        let filter: Vec<Box<dyn Fn(&str) -> bool>> = vec![Box::new(filter0), Box::new(filter1)];
        let folder = MultiLayerFolder::new(file, filter);
        let mut records = Vec::new();

        for file in folder {
            let date = extract_d10(&file.name).unwrap();
            let date = format!("20{}", date);
            let date = parse_yyyymmddhhmm(&date).unwrap();

            let radolan = RadolanFile::new(file.data);
            let precision = radolan.header.precision;
            let parsed = radolan.extract_points(&request_data.coordinates);
            let parsed = parsed
                .into_iter()
                .map(|(_, v)| v.default_f32(precision))
                .collect::<Vec<_>>();

            records.push(Record {
                data: parsed,
                time: date,
            });
        }
        records
    }
}

pub struct Recent;

impl dwd_source::DwdSource for Recent {
    type Record = Record;
    type RequestData = RadolanRequest;

    fn urls(&self, _: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}/climate_environment/CDC/grids_germany/hourly/radolan/recent/bin/",
            base_url()
        );

        let html = download_text(&url, None);
        let regex = r"raa01-rw_10000-\d{10}-dwd---bin.gz";
        let links = links_in_text(&html, &regex);

        let links = links
            .iter()
            .map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: Some({
                    let date = extract_d10(link).unwrap();
                    let date = format!("20{}", date);
                    let date = parse_yyyymmddhhmm(&date).unwrap();
                    Interval::new(date, date).unwrap()
                }),
            })
            .collect();

        dbg!(&links);
        links
    }

    fn extract_data(&self, request_data: &Self::RequestData, file: File) -> Vec<Self::Record> {
        let filter0 = |_: &str| true;
        let filter: Vec<Box<dyn Fn(&str) -> bool>> = vec![Box::new(filter0)];
        let folder = MultiLayerFolder::new(file, filter);
        let mut records = Vec::new();

        for file in folder {
            let date = extract_d10(&file.name).unwrap();
            let date = format!("20{}", date);
            let date = parse_yyyymmddhhmm(&date).unwrap();

            let radolan = RadolanFile::new(file.data);
            let precision = radolan.header.precision;
            let parsed = radolan.extract_points(&request_data.coordinates);
            let parsed = parsed
                .into_iter()
                .map(|(_, v)| v.default_f32(precision))
                .collect::<Vec<_>>();

            records.push(Record {
                data: parsed,
                time: date,
            });
        }
        records
    }
}

fn year_month_to_interval(date: Date) -> Interval<PrimitiveDateTime> {
    let last_month_day = days_in_year_month(date.year(), date.month());
    let end = date.replace_day(last_month_day).unwrap();
    Interval::new(date, end).unwrap().into()
}
