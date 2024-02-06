use dwd_dl::{
    dwd_source::{CommonRequestData, DwdProduct},
    products::precipitation::{
        self, PrecipitationCommonRequestData, PrecipitationResolution, PrecipitationResponse,
    },
};
use time::macros::datetime;

mod common;

#[test]
fn test_min1() {
    common::setup();
    let resolution = PrecipitationResolution::PrecipitationMin1;
    let request = PrecipitationCommonRequestData {
        station: "00020".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 01 - 31 23:57:00),
                end: datetime!(2022 - 02 - 01 00:03:00),
            },
        },
    };
    let response = precipitation::Product.downloadx(request);

    let assert = PrecipitationResponse {
        station: "00020".to_string(),
        records: vec![
            precipitation::PrecipitationRecord {
                timespan: dwd_dl::util::interval::Interval::new(
                    datetime!(2022 - 01 - 31 23:57:00),
                    datetime!(2022 - 01 - 31 23:57:00),
                )
                .unwrap(),
                rs: 0.0,
            },
            precipitation::PrecipitationRecord {
                timespan: dwd_dl::util::interval::Interval::new(
                    datetime!(2022 - 01 - 31 23:58:00),
                    datetime!(2022 - 01 - 31 23:58:00),
                )
                .unwrap(),
                rs: 0.0,
            },
            precipitation::PrecipitationRecord {
                timespan: dwd_dl::util::interval::Interval::new(
                    datetime!(2022 - 01 - 31 23:59:00),
                    datetime!(2022 - 01 - 31 23:59:00),
                )
                .unwrap(),
                rs: 0.0,
            },
            precipitation::PrecipitationRecord {
                timespan: dwd_dl::util::interval::Interval::new(
                    datetime!(2022 - 02 - 01 00:00:00),
                    datetime!(2022 - 02 - 01 00:59:00),
                )
                .unwrap(),
                rs: 0.0,
            },
        ],
    };
    assert_eq!(response, assert);
}

#[test]
fn test_min5() {
    common::setup();
    let resolution = PrecipitationResolution::PrecipitationMin5;
    let request = PrecipitationCommonRequestData {
        station: "00020".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval::new(
                datetime!(2008 - 03 - 31 23:55),
                datetime!(2008 - 04 - 01 00:06),
            )
            .unwrap(),
        },
    };
    let response = precipitation::Product.downloadx(request);
    // dbg!(&response.records);
    assert_eq!(response.records.len(), 3);
}

#[test]
fn test_min10() {
    common::setup();
    let resolution = PrecipitationResolution::PrecipitationMin10;
    let request = PrecipitationCommonRequestData {
        station: "00078".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval::new(
                datetime!(2009 - 12 - 31 23:40),
                datetime!(2010 - 01 - 01 01:06),
            )
            .unwrap(),
        },
    };
    let response = precipitation::Product.downloadx(request);
    dbg!(&response.records);
    assert_eq!(response.records.len(), 9);
}

#[test]
fn test_hourly() {
    common::setup();
    let resolution = PrecipitationResolution::PrecipitationHourly;
    let request = PrecipitationCommonRequestData {
        station: "00164".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval::new(
                datetime!(2022 - 12 - 31 19:55),
                datetime!(2023 - 01 - 01 01:06),
            )
            .unwrap(),
        },
    };
    let response = precipitation::Product.downloadx(request);
    dbg!(&response.records);
    assert_eq!(response.records.len(), 6);
}
