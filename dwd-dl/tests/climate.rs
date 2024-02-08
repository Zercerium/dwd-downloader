use dwd_dl::{
    dwd_source::{CommonRequestData, DwdProduct},
    products::climate::{
        climate_data_to_string, ClimateCommonRequestData, ClimateProduct, ClimateResolution,
    },
};
use time::macros::datetime;

mod common;

#[test]
fn test_daily() {
    common::setup();
    let resolution = ClimateResolution::ClimateDaily;
    let request = ClimateCommonRequestData {
        station: "04271".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 12 - 25 00:00:00),
                end: datetime!(2023 - 01 - 06 00:00:00),
            },
        },
    };
    let data = ClimateProduct.download(&request);
    let response = climate_data_to_string(data, &resolution);

    let assert = r"STATIONS_ID;MESS_DATUM;QN_3;  FX;  FM;QN_4; RSK;RSKF; SDK;SHK_TAG;  NM; VPM;  PM; TMK; UPM; TXK; TNK; TGK;eor
       4271;20221225;    3;   8.7;   3.5;    3;   4.6;   6;    0.000;   0;   7.0;   8.5; 1012.31;    5.4;   95.13;    7.2;    2.7;    2.2;eor
       4271;20221226;    3;  18.8;   4.8;    3;   0.1;   6;    0.050;   0;   7.3;   8.7; 1006.82;    6.4;   89.08;    9.4;    4.0;    2.1;eor
       4271;20221227;    3;  14.4;   5.1;    3;   0.5;   6;    4.417;   0;   4.4;   6.6; 1015.60;    3.8;   82.79;    4.7;    2.9;    0.9;eor
       4271;20221228;    3;  16.1;   5.9;    3;   4.3;   6;    0.000;   0;   7.5;   8.6; 1006.65;    6.5;   88.08;    8.5;    3.8;    3.1;eor
       4271;20221229;    3;  15.1;   5.9;    3;   4.2;   6;    1.300;   0;   6.8;   9.4;  998.71;    8.8;   83.04;   11.0;    6.0;    4.8;eor
       4271;20221230;    3;  16.7;   5.3;    3;   1.4;   6;    2.583;   0;   6.8;   8.1; 1004.10;    7.1;   80.13;    9.2;    6.0;    4.1;eor
       4271;20221231;    3;  12.9;   5.3;    3;   6.5;   6;    0.000;   0;   7.5;  12.4; 1001.31;   11.9;   88.75;   15.0;    9.3;    8.0;eor
       4271;20230101;   10;  16.8;   5.5;    3;   1.7;   6;    3.350;   0;   6.8;  10.2; 1008.61;   13.1;   67.83;   15.8;    9.9;    8.0;eor
       4271;20230102;   10;  15.7;   5.2;    3;   0.4;   6;    0.000;   0;   7.6;  10.7; 1011.07;   10.1;   86.33;   13.6;    6.9;    5.2;eor
       4271;20230103;   10;  13.3;   4.0;    3;   0.8;   6;    5.300;   0;   2.6;   7.0; 1022.79;    4.9;   81.08;    7.0;    2.8;    0.2;eor
       4271;20230104;   10;  17.0;   5.9;    3;  19.4;   6;    0.000;   0;   7.2;   9.7; 1007.20;    8.1;   88.38;   11.5;    4.9;    3.9;eor
       4271;20230105;   10;  22.6;   9.9;    3;   0.5;   6;    0.000;   0;   7.8;   7.5; 1009.75;    6.0;   79.38;    9.9;    3.1;   -1.4;eor
";
    assert_eq!(response, assert);
}

#[test]
fn test_monthly() {
    common::setup();
    let resolution = ClimateResolution::ClimateMonthly;
    let data = ClimateProduct.download(&ClimateCommonRequestData {
        station: "00044".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2022 - 03 - 01 00:00:00),
                end: datetime!(2023 - 05 - 02 00:00:00),
            },
        },
    });
    let response = climate_data_to_string(data, &resolution);

    let assert = r"STATIONS_ID;MESS_DATUM_BEGINN;MESS_DATUM_ENDE;QN_4;MO_N;MO_TT;MO_TX;MO_TN;MO_FK;MX_TX;MX_FX;MX_TN;MO_SD_S;QN_6;MO_RR;MX_RS;eor
         44;20220301;20220331;    3;  -999;   5.90;   11.87;    0.25;-999;  18.9;-999;  -3.3; 234.83;    3;    21.2;   11.8;eor
         44;20220401;20220430;    3;  -999;   8.31;   13.46;    3.18;-999;  21.5;-999;  -2.3; 194.97;    3;    40.2;   12.9;eor
         44;20220501;20220531;    3;  -999;  13.90;   19.93;    7.58;-999;  29.9;-999;   0.8; 224.07;    3;    49.0;   11.0;eor
         44;20220601;20220630;    3;  -999;  16.95;   23.13;   10.17;-999;  31.3;-999;   5.3; 267.57;    3;    41.2;    8.5;eor
         44;20220701;20220731;    3;  -999;  18.05;   24.08;   11.80;-999;  36.8;-999;   7.1; 213.37;    3;    67.8;   25.6;eor
         44;20220801;20220831;    3;  -999;  19.98;   27.07;   12.76;-999;  33.9;-999;   7.8; 277.27;    3;    26.2;   12.5;eor
         44;20220901;20220930;    3;  -999;  13.91;   19.28;    9.30;-999;  29.0;-999;   3.6; 169.80;    3;    92.7;   18.5;eor
         44;20221001;20221031;    3;  -999;  12.89;   17.43;    8.56;-999;  22.3;-999;   2.6; 154.17;    3;    20.7;    4.4;eor
         44;20221101;20221130;    3;  -999;   7.43;   10.33;    4.54;-999;  16.8;-999;  -4.2;  81.36;    3;    48.8;   10.5;eor
         44;20221201;20221231;    3;  -999;   2.82;    5.09;    0.19;-999;  15.8;-999;  -9.4;  37.05;    1;    84.7;   19.5;eor
         44;20230101;20230131;    3;  -999;   5.00;    6.90;    2.62;-999;  15.6;-999;  -3.1;  30.89;    3;   104.5;   19.6;eor
         44;20230201;20230228;    3;  -999;   4.76;    8.19;    1.31;-999;  12.4;-999;  -5.6;  90.92;    3;    42.2;   16.6;eor
         44;20230301;20230331;    3;  -999;   6.11;    9.58;    2.90;-999;  17.0;-999;  -5.1;  91.85;    3;    83.8;   14.4;eor
         44;20230401;20230430;    3;  -999;   8.27;   13.33;    3.03;-999;  22.8;-999;  -3.0; 192.27;    3;    59.6;   12.1;eor
         44;20230501;20230531;    3;  -999;  12.80;   18.33;    7.38;-999;  25.4;-999;  -0.2; 229.10;    3;    48.3;   17.4;eor
";
    assert_eq!(response, assert);
}

#[test]
fn test_annual() {
    common::setup();
    let resolution = ClimateResolution::ClimateAnnual;
    let data = ClimateProduct.download(&ClimateCommonRequestData {
        station: "00044".to_string(),
        resolution,
        common: CommonRequestData {
            timespan: dwd_dl::util::interval::Interval {
                start: datetime!(2020 - 01 - 01 00:00:00),
                end: datetime!(2023 - 10 - 01 00:00:00),
            },
        },
    });
    let response = climate_data_to_string(data, &resolution);

    let assert = r"STATIONS_ID;MESS_DATUM_BEGINN;MESS_DATUM_ENDE;QN_4;JA_N;JA_TT;JA_TX;JA_TN;JA_FK;JA_SD_S;JA_MX_FX;JA_MX_TX;JA_MX_TN;QN_6;JA_RR;JA_MX_RS;eor
         44;20200101;20201231;   10;  -999;  10.89;   15.42;    6.36;-999;-999;-999;  35.0;  -4.9;-999;-999;-999;eor
         44;20210101;20211231;    3;  -999;   9.81;   14.04;    5.83;-999; 1477.14;-999;  34.3; -16.7;    9;   739.4;   19.9;eor
         44;20220101;20221231;    3;  -999;  10.90;   15.61;    6.16;-999; 1966.36;-999;  36.8;  -9.4;    1;   672.3;   25.6;eor
         44;20230101;20231231;    1;  -999;  10.96;   15.15;    6.85;-999; 1646.64;-999;  32.5;  -5.7;    1;  1124.2;   38.2;eor
";
    assert_eq!(response, assert);
}
