use time::{util::days_in_year_month, Date, PrimitiveDateTime};

use crate::{
    base_url,
    dwd_source::{self, UrlTimeIntervall},
    products::evaporation::{decode::decode, EvaporationRequest, Record},
    util::{
        compression::universal::MultiLayerFolder,
        download::download_text,
        file::File,
        interval::Interval,
        regex::{extract_d6, links_in_text},
        time::parse_yyyymm,
    },
};

pub struct MonthlyP;

impl dwd_source::DwdSource for MonthlyP {
    type Record = Record;
    type RequestData = EvaporationRequest;

    fn urls(&self, _: &Self::RequestData) -> Vec<UrlTimeIntervall> {
        let url = format!(
            "{}/climate_environment/CDC/grids_germany/monthly/evapo_p/",
            base_url()
        );
        let html = download_text(&url, None);
        let regex = r"grids_germany_monthly_evapo_p_\d{6}.asc.gz";
        let links = links_in_text(&html, regex);

        dbg!(&links);

        let res = links
            .iter()
            .map(|link| UrlTimeIntervall {
                url: format!("{}{}", url, link),
                interval: Some({
                    let date = extract_d6(link).unwrap();
                    let date = parse_yyyymm(&date).unwrap();
                    year_month_to_interval(date)
                }),
            })
            .collect();

        dbg!(&res);
        res
    }

    fn extract_data(&self, request_data: &Self::RequestData, file: File) -> Vec<Self::Record> {
        let filter0 = |_: &str| true;
        let filter: Vec<Box<dyn Fn(&str) -> bool>> = vec![Box::new(filter0)];
        let folder = MultiLayerFolder::new(file, filter);
        let mut records = Vec::new();

        for file in folder {
            let date = extract_d6(&file.name).unwrap();
            let date = parse_yyyymm(date).unwrap();
            let ascii = std::str::from_utf8(&file.data).unwrap();
            let parsed = decode(ascii, &request_data.coordinates);

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
