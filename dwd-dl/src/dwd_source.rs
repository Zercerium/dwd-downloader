use std::fmt::Debug;

use time::PrimitiveDateTime;

use crate::util::{
    download::{create_client, download_body, download_file},
    file::File,
    interval::{Interval, Overlaps},
};

#[derive(Debug, Clone)]
pub struct CommonRequestData {
    pub timespan: Interval<PrimitiveDateTime>,
}

#[derive(Debug)]
pub struct UrlTimeIntervall {
    pub url: String,
    pub interval: Option<Interval<PrimitiveDateTime>>,
}

pub trait Timespan {
    fn timespan(&self) -> Interval<PrimitiveDateTime>;
}

pub trait Common {
    fn common(&self) -> &CommonRequestData;
}

pub trait Sources
where
    Self: Common,
{
    type Record: Timespan + Debug;

    fn sources(&self) -> Vec<Box<dyn DwdSource<Record = Self::Record, RequestData = Self>>>;
}

pub trait DwdSource {
    type Record: Timespan + Debug;
    type RequestData: Common;

    fn urls(&self, request_data: &Self::RequestData) -> Vec<UrlTimeIntervall>;

    fn extract_data(&self, request_data: &Self::RequestData, file: File) -> Vec<Self::Record>;

    fn send(&self, request_data: &Self::RequestData) -> Vec<Self::Record> {
        let client = create_client();
        let urls = self.urls(request_data);
        dbg!(&urls);
        urls.iter()
            .inspect(|x| println!("BEFORE: {:?}", x.url))
            .filter(|url| {
                url.interval
                    .map_or(true, |i| i.overlaps(&request_data.common().timespan))
            })
            .inspect(|x| println!("FILTER: {:?}", x.url))
            .map(|url| download_file(&url.url, Some(&client)))
            .flat_map(|file| self.extract_data(request_data, file))
            // .inspect(|x| println!("{:?}", x.timespan()))
            .skip_while(|d| d.timespan().start < request_data.common().timespan.start)
            .take_while(|d| d.timespan().start < request_data.common().timespan.end)
            .collect::<Vec<_>>()
    }
}

pub trait DwdProduct {
    type Request: Common + Sources;
    type Response;

    fn download(
        &self,
        request: &Self::Request,
    ) -> Vec<<<Self as DwdProduct>::Request as Sources>::Record> {
        let mut last_timestamp = None;
        let mut records = Vec::new();
        let sources = request.sources();
        for source in sources {
            if last_timestamp.is_some_and(|l| l >= request.common().timespan.end) {
                break;
            }
            let current_records = source.send(&request);
            let mut iter = current_records
                .into_iter()
                .skip_while(|d| match last_timestamp {
                    Some(l) => l >= d.timespan().start,
                    None => request.common().timespan.start > d.timespan().start,
                });
            records.extend(&mut iter);
            if let Some(last) = records.last() {
                last_timestamp = Some(last.timespan().end);
            };
        }
        records
    }

    fn downloadx(&self, request: Self::Request) -> Self::Response;
}
