use dwd_dl::{
    dwd_source::{CommonRequestData, DwdProduct},
    products::evaporation::{
        self,
        formats::{format_default, format_swmm_rainfall_data},
        EvaporationRequest, EvaporationResolution,
    },
    util::point::Point,
};
use time::macros::{date, datetime};

mod common;

#[test]
fn test_daily_p() {
    common::setup();
    let resolution = EvaporationResolution::EvaporationDailyP;
    let request = EvaporationRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 31 00:00:00),
                end: datetime!(2023 - 01 - 05 00:00:00),
            },
        },
    };
    let response = evaporation::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 5);
}

#[test]
fn test_daily_r() {
    common::setup();
    let resolution = EvaporationResolution::EvaporationDailyR;
    let request = EvaporationRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 15 00:00:00),
                end: datetime!(2022 - 12 - 21 00:00:00),
            },
        },
    };
    let response = evaporation::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 6);
}

#[test]
fn test_monthly_p() {
    common::setup();
    let resolution = EvaporationResolution::EvaporationMonthlyP;
    let request = EvaporationRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 1 00:00:00),
                end: datetime!(2022 - 12 - 2 00:00:00),
            },
        },
    };
    let response = evaporation::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 1);
}

#[test]
fn test_monthly_r() {
    common::setup();
    let resolution = EvaporationResolution::EvaporationMonthlyR;
    let request = EvaporationRequest {
        coordinates: vec![Point::new(20, 369), Point::new(21, 369)],
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 1 00:00:00),
                end: datetime!(2023 - 12 - 2 00:00:00),
            },
        },
    };
    let response = evaporation::Product.downloadx(request);
    dbg!(&response);
    assert_eq!(response.records.len(), 1);
}

#[test]
fn test_format_default() {
    common::setup();

    let res = format_default(evaporation::EvaporationResponse {
        coordinates: vec![
            Point::new(4, 1), // 1
            Point::new(4, 2), // 3
            Point::new(5, 2), // 4
            Point::new(7, 1), // 2
        ],
        records: vec![
            evaporation::Record {
                time: date!(2022 - 12 - 31),
                data: vec![1.0, 2.0, 3.0, 4.0],
            },
            evaporation::Record {
                time: date!(2023 - 01 - 01),
                data: vec![1.1, 2.1, 3.1, 4.1],
            },
            evaporation::Record {
                time: date!(2023 - 01 - 02),
                data: vec![1.2, 2.2, 3.2, 4.2],
            },
            evaporation::Record {
                time: date!(2023 - 01 - 03),
                data: vec![1.3, 2.3, 3.3, 4.3],
            },
        ],
    });

    assert_eq!(
        "x_y\tDate\tValue\n0004_0001\t2022-12-31\t1.0\n0004_0001\t2023-01-01\t1.1\n0004_0001\t2023-01-02\t1.2\n0004_0001\t2023-01-03\t1.3\n0004_0002\t2022-12-31\t3.0\n0004_0002\t2023-01-01\t3.1\n0004_0002\t2023-01-02\t3.2\n0004_0002\t2023-01-03\t3.3\n0005_0002\t2022-12-31\t4.0\n0005_0002\t2023-01-01\t4.1\n0005_0002\t2023-01-02\t4.2\n0005_0002\t2023-01-03\t4.3\n0007_0001\t2022-12-31\t2.0\n0007_0001\t2023-01-01\t2.1\n0007_0001\t2023-01-02\t2.2\n0007_0001\t2023-01-03\t2.3\n",
        res
    );
}

#[test]
fn test_format_swmm_rainfall_data() {
    common::setup();

    let res = format_swmm_rainfall_data(evaporation::EvaporationResponse {
        coordinates: vec![Point::new(2, 1)],
        records: vec![
            evaporation::Record {
                time: date!(2022 - 12 - 31),
                data: vec![1.0],
            },
            evaporation::Record {
                time: date!(2023 - 01 - 01),
                data: vec![2.0],
            },
            evaporation::Record {
                time: date!(2023 - 01 - 02),
                data: vec![3.0],
            },
            evaporation::Record {
                time: date!(2023 - 01 - 03),
                data: vec![4.0],
            },
        ],
    });

    assert_eq!(
        "MM/DD/YYYY\thh:mm\tValue\n12/31/2022\t00:00\t1.0\n01/01/2023\t00:00\t2.0\n01/02/2023\t00:00\t3.0\n01/03/2023\t00:00\t4.0\n",
        res
    );
}
