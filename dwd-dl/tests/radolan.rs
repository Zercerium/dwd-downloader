use dwd_dl::{
    dwd_source::{CommonRequestData, DwdProduct},
    products::radolan::{
        self,
        formats::{format_default, format_swmm_rainfall_data, RadolanFormatConfig},
        RadolanRequest, RadolanResolution,
    },
    util::point::Point,
};
use time::macros::datetime;

mod common;

#[test]
fn test_min5() {
    common::setup();
    let resolution = RadolanResolution::RadolanMin5;
    let request = RadolanRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2023 - 11 - 17 23:30:00),
                end: datetime!(2023 - 11 - 18 00:30:00),
            },
        },
    };
    let response = radolan::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 12);
}

#[test]
fn test_min5_reproc() {
    common::setup();
    let resolution = RadolanResolution::RadolanMin5Reproc2017;
    let request = RadolanRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 31 23:30:00),
                end: datetime!(2023 - 01 - 01 00:30:00),
            },
        },
    };
    let response = radolan::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 12);
}

#[test]
fn test_hourly() {
    common::setup();
    let resolution = RadolanResolution::RadolanHourly;
    let request = RadolanRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 31 00:00:00),
                end: datetime!(2023 - 01 - 01 03:00:00),
            },
        },
    };
    let response = radolan::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 27);
}

#[test]
fn test_hourly_reproc() {
    common::setup();
    let resolution = RadolanResolution::RadolanHourlyReproc2017;
    let request = RadolanRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 31 00:00:00),
                end: datetime!(2023 - 01 - 01 06:00:00),
            },
        },
    };
    let response = radolan::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 30);
}

#[test]
fn test_daily() {
    common::setup();
    let resolution = RadolanResolution::RadolanDaily;
    let request = RadolanRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 31 00:00:00),
                end: datetime!(2023 - 01 - 01 06:00:00),
            },
        },
    };
    let response = radolan::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 30);
}

#[test]
fn test_format_default() {
    common::setup();

    let res = format_default(
        radolan::RadolanResponse {
            coordinates: vec![
                Point::new(4, 1), // 1
                Point::new(4, 2), // 3
                Point::new(5, 2), // 4
                Point::new(7, 1), // 2
            ],
            records: vec![
                radolan::Record {
                    time: datetime!(2022 - 12 - 31 00:00:00),
                    data: vec![1.0, 2.0, 3.0, 4.0],
                },
                radolan::Record {
                    time: datetime!(2023 - 01 - 01 00:00:00),
                    data: vec![1.1, 2.1, 3.1, 4.1],
                },
                radolan::Record {
                    time: datetime!(2023 - 01 - 02 00:00:00),
                    data: vec![1.2, 2.2, 3.2, 4.2],
                },
                radolan::Record {
                    time: datetime!(2023 - 01 - 03 00:00:00),
                    data: vec![1.3, 2.3, 3.3, 4.3],
                },
            ],
        },
        RadolanFormatConfig {
            utc_to_berlin: false,
            offset: 0,
        },
    );

    let cmp = "Name\tDate\tTime\tValue\n\
    0004_0001\t2022-12-31\t00:00\t1.00\n\
    0004_0001\t2023-01-01\t00:00\t1.10\n\
    0004_0001\t2023-01-02\t00:00\t1.20\n\
    0004_0001\t2023-01-03\t00:00\t1.30\n\
    0004_0002\t2022-12-31\t00:00\t3.00\n\
    0004_0002\t2023-01-01\t00:00\t3.10\n\
    0004_0002\t2023-01-02\t00:00\t3.20\n\
    0004_0002\t2023-01-03\t00:00\t3.30\n\
    0005_0002\t2022-12-31\t00:00\t4.00\n\
    0005_0002\t2023-01-01\t00:00\t4.10\n\
    0005_0002\t2023-01-02\t00:00\t4.20\n\
    0005_0002\t2023-01-03\t00:00\t4.30\n\
    0007_0001\t2022-12-31\t00:00\t2.00\n\
    0007_0001\t2023-01-01\t00:00\t2.10\n\
    0007_0001\t2023-01-02\t00:00\t2.20\n\
    0007_0001\t2023-01-03\t00:00\t2.30\n";

    assert_eq!(res.lines().count(), cmp.lines().count());

    res.lines()
        .zip(cmp.lines())
        .for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn test_format_swmm_rainfall_data() {
    common::setup();

    let res = format_swmm_rainfall_data(
        radolan::RadolanResponse {
            coordinates: vec![
                Point::new(4, 1), // 1
                Point::new(4, 2), // 3
                Point::new(5, 2), // 4
                Point::new(7, 1), // 2
            ],
            records: vec![
                radolan::Record {
                    time: datetime!(2022 - 12 - 31 01:20:00),
                    data: vec![1.0, 2.0, 3.0, 4.0],
                },
                radolan::Record {
                    time: datetime!(2023 - 01 - 01 02:20:00),
                    data: vec![1.1, 2.1, 3.1, 4.1],
                },
                radolan::Record {
                    time: datetime!(2023 - 01 - 02 03:00:00),
                    data: vec![1.2, 2.2, 3.2, 4.2],
                },
                radolan::Record {
                    time: datetime!(2023 - 01 - 03 03:10:00),
                    data: vec![1.3, 2.3, 3.3, 4.3],
                },
            ],
        },
        RadolanFormatConfig {
            utc_to_berlin: true,
            offset: -10,
        },
    );

    let cmp = "Name\tJahr\tMonat\tTag\tStunde\tMinute\tWert\n\
    0004_0001\t2022\t12\t31\t02\t10\t1.00\n\
    0004_0001\t2023\t01\t01\t03\t10\t1.10\n\
    0004_0001\t2023\t01\t02\t03\t50\t1.20\n\
    0004_0001\t2023\t01\t03\t04\t00\t1.30\n\
    0004_0002\t2022\t12\t31\t02\t10\t3.00\n\
    0004_0002\t2023\t01\t01\t03\t10\t3.10\n\
    0004_0002\t2023\t01\t02\t03\t50\t3.20\n\
    0004_0002\t2023\t01\t03\t04\t00\t3.30\n\
    0005_0002\t2022\t12\t31\t02\t10\t4.00\n\
    0005_0002\t2023\t01\t01\t03\t10\t4.10\n\
    0005_0002\t2023\t01\t02\t03\t50\t4.20\n\
    0005_0002\t2023\t01\t03\t04\t00\t4.30\n\
    0007_0001\t2022\t12\t31\t02\t10\t2.00\n\
    0007_0001\t2023\t01\t01\t03\t10\t2.10\n\
    0007_0001\t2023\t01\t02\t03\t50\t2.20\n\
    0007_0001\t2023\t01\t03\t04\t00\t2.30\n";

    assert_eq!(res.lines().count(), cmp.lines().count());

    res.lines()
        .zip(cmp.lines())
        .for_each(|(a, b)| assert_eq!(a, b));
}
