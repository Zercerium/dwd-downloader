use bytes::Bytes;

use crate::util::file::File;

use super::{gz::decode_gz, tar::Tarball};

pub struct MultiLayerFolder {
    file: File,
    filters: Vec<Box<dyn Fn(&str) -> bool>>,
}

impl MultiLayerFolder {
    pub fn new(file: File, filters: Vec<Box<dyn Fn(&str) -> bool>>) -> Self {
        Self { file, filters }
    }
}

impl IntoIterator for MultiLayerFolder {
    type Item = File;

    type IntoIter = MultiLayerFolderIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut data = Vec::new();
        data.push(File::new(self.file.name, self.file.data));

        MultiLayerFolderIter {
            folder: data,
            current_layer: 0,
            layer_start_index: vec![0],
            filters: self.filters,
        }
    }
}

pub struct MultiLayerFolderIter {
    folder: Vec<File>,
    current_layer: usize,
    layer_start_index: Vec<usize>,
    filters: Vec<Box<dyn Fn(&str) -> bool>>,
}

impl Iterator for MultiLayerFolderIter {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // for i in self.folder.iter() {
            //     println!("{}", i.filename);
            // }
            if self.folder.is_empty() {
                return None;
            }
            if self.folder.len() < *self.layer_start_index.last().unwrap() {
                self.layer_start_index.pop();
                self.current_layer -= 1;
            }
            // println!("Layer: {}", self.current_layer);

            let file = self.folder.last().unwrap();

            let filter = &self.filters[self.current_layer];
            if !filter(&file.name) {
                let _ = self.folder.pop().unwrap();
                continue;
            }

            let extension = file.extension().unwrap().into();
            println!("Processing: {}", file.name);
            match extension {
                Extension::Gz => {
                    let file = self.folder.pop().unwrap();
                    let decoded = decode_gz(&file.data);
                    let file = File::new(
                        file.name.strip_suffix(".gz").unwrap().into(),
                        Bytes::from(decoded),
                    );
                    self.folder.push(file);
                }
                Extension::Tar => {
                    self.current_layer += 1;
                    self.layer_start_index.push(self.folder.len());
                    let file = self.folder.pop().unwrap();
                    let file = Tarball::new(file.data);
                    for entry in file.entries().iter().rev() {
                        let file = entry.data.clone();
                        let file = File::new(entry.filename.clone(), file);
                        self.folder.push(file);
                    }
                }
                Extension::Tgz => {
                    let mut file = self.folder.pop().unwrap();
                    file.name = file.name.strip_suffix(".tgz").unwrap().into();
                    file.name.push_str(".tar.gz");
                    self.folder.push(file);
                }
                Extension::Other => {
                    let file = self.folder.pop().unwrap();
                    return Some(file);
                }
            }
        }
    }
}

enum Extension {
    Gz,
    Tar,
    Tgz,
    Other,
}

impl From<&str> for Extension {
    fn from(s: &str) -> Self {
        match s {
            "gz" => Extension::Gz,
            "tar" => Extension::Tar,
            "tgz" => Extension::Tgz,
            _ => Extension::Other,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use time::{
        macros::{date, datetime, format_description},
        PrimitiveDateTime,
    };

    use crate::util::{
        compression::{gz::decode_gz, tar::Tarball},
        interval::Interval,
    };

    use super::*;

    #[test]
    fn test() {
        let file = fs::read("../example_setup/ftp_data/climate_environment/CDC/grids_germany/daily/evapo_p/grids_germany_daily_evapo_p_202210.tgz").unwrap();
        let file = decode_gz(&file);
        let file = Tarball::new(file.into());
        for entry in file.entries() {
            println!("{}", entry.filename);
        }
        let content = file.entries()[0].data.clone();
        let content = std::str::from_utf8(&content).unwrap();
        println!("{}", content);
    }

    #[test]
    fn test3() {
        let interval: Interval<PrimitiveDateTime> =
            Interval::new(date!(2022 - 11 - 26), date!(2022 - 11 - 26))
                .unwrap()
                .into();
        let filter0 = |_: &str| true;
        let filter1 = move |s: &str| {
            // println!("Filter1: {}", s);
            let regex = regex::Regex::new(r"\d{8}").unwrap();
            let cap = regex.captures(s).unwrap();
            let date = cap[0].to_string();
            let format = format_description!("[year][month][day]");
            let date = time::Date::parse(&date, &format).unwrap();

            interval.contains(&date.midnight())
        };

        let filter2 = |s: &str| {
            // println!("Filter2: {}", s);
            let regex = regex::Regex::new(r"\d{10}").unwrap();
            let cap = regex.captures(s).unwrap();
            let date = format!("20{}", &cap[0]);
            let format = format_description!("[year][month][day][hour][minute]");
            let date = time::PrimitiveDateTime::parse(&date, &format).unwrap();
            let interval =
                Interval::new(datetime!(2022-11-26 04:00), datetime!(2022-11-26 05:00)).unwrap();
            interval.contains(&date)
        };

        let filters: Vec<Box<dyn Fn(&str) -> bool>> =
            vec![Box::new(filter0), Box::new(filter1), Box::new(filter2)];

        let name = "YW2017.002_202211.tar";
        let file = fs::read(format!(
            "../example_setup/ftp_data/climate_environment/CDC/grids_germany/5_minutes/radolan/reproc/2017_002/bin/2022/{}",
            name
        ))
        .unwrap();
        let file = File::new(name.into(), file.into());
        let folder = MultiLayerFolder::new(file, filters);
        let folder: Vec<_> = folder.into_iter().collect();
        for file in &folder {
            println!("{}", file.name);
        }
        assert_eq!(folder.len(), 12);
    }
}
