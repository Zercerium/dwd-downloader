use std::{error::Error, str::FromStr};

use nom::{
    bytes::complete::{tag, take},
    character::complete::digit0,
    combinator::opt,
    IResult,
};
use time::{format_description::well_known::Iso8601, PrimitiveDateTime};

use crate::util::point::Point;

pub struct RadolanFile<'a> {
    pub header: Header,
    pub data: &'a [u8],
}

impl<'a> RadolanFile<'a> {
    pub fn new(file: &'a [u8]) -> Self {
        let header: Vec<_> = file.iter().map(|&b| b).take_while(|&b| b != 0x03).collect();

        let header_end = file.iter().position(|b| *b == 0x03).unwrap();
        let header_str = String::from_utf8(header).unwrap();
        let data = &file[header_end + 1..];

        let header = header_str.parse().unwrap();

        RadolanFile { header, data }
    }

    /// returns the values at the given points from the file
    pub fn extract_points(&self, points: &Vec<Point<u16>>) -> Vec<(Point<u16>, BType)> {
        points
            .iter()
            .map(|p| {
                let offset = offset_bottom_left_1_1(
                    p.column,
                    p.row,
                    self.header.dimension.rows,
                    self.header.dimension.columns,
                ) * 2;
                (
                    *p,
                    parse(&self.data[offset..offset + 2].try_into().unwrap()),
                )
            })
            .collect()
    }
}

///calculate the offset where the first point in the file is the bottom left point and has the coordinates 1,1
fn offset_bottom_left_1_1(x: u16, y: u16, rows: u16, columns: u16) -> usize {
    (rows - y) as usize * columns as usize + (x - 1) as usize
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BType {
    Normal(u16),       //0000
    Interpolated(u16), //0001 or Hail
    Error(u16),        //0010
    Neg(u16),          //0100
    Scope(u16),        //1000
}

pub fn parse(data: &[u8; 2]) -> BType {
    let data_type = data[1] >> 4;
    let value = ((data[1] & 0b0000_1111) as u16) << 8 | data[0] as u16;
    match data_type {
        0b0000 => BType::Normal(value),
        0b0001 => BType::Interpolated(value),
        0b0010 => BType::Error(value),
        0b0100 => BType::Neg(value),
        0b1000 => BType::Scope(value),
        _ => unreachable!("Unknown data type"),
    }
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub produktkennung: Produktkennung,
    pub datetime: PrimitiveDateTime,
    pub location: String,    //radarstandort
    pub product_length: u32, //produktlänge
    pub format_version: String,
    pub software_version: String,
    pub precision: u8,          //genauigkeit
    pub interval_duration: u16, //intervalldauer
    pub unit: Option<u8>,       //masseinheit
    pub dimension: Dimension,
    // MF = 00000001 für die erste Version der Qualität dieser Produkte steht.
    pub binary_representation: Option<u32>, //binäre Darstellung
    pub radar_locations: Vec<String>,       //standorte
    pub radar_location_contributions: Option<Vec<(String, u8)>>, //standort_beitrag
}

#[derive(Debug, PartialEq)]
pub struct Dimension {
    pub rows: u16,
    pub columns: u16,
}

#[derive(Debug, PartialEq)]
pub enum Produktkennung {
    YW,
    RW,
    SF,
}

impl FromStr for Header {
    type Err = Box<dyn Error>;

    fn from_str<'a>(s: &'a str) -> Result<Self, Self::Err> {
        let (s, produktkennung) = parse_produktkennung(s).unwrap();
        let (s, (datetime, location)) = parse_datetime_and_location(s).unwrap();
        let (s, product_length) = parse_product_length(s).unwrap();
        let (s, format_version) = parse_format_version(s).unwrap();
        let (s, software_version) = parse_software_version(s).unwrap();
        let (s, precision) = parse_precision(s).unwrap();
        let (s, interval_duration) = parse_interval_duration(s).unwrap();
        let (s, unit) = parse_unit(s).unwrap();
        let (s, dimension) = parse_dimension(s).unwrap();
        let (s, binary_representation) = parse_binary_representation(s).unwrap();
        let (s, _) = parse_undocumented_vr(s).unwrap();
        let (s, radar_locations) = parse_radar_locations(s).unwrap();
        let (s, radar_location_contributions) = parse_radar_location_contributions(s).unwrap();
        assert!(s.is_empty());

        Ok(Self {
            produktkennung,
            datetime,
            location,
            product_length,
            format_version,
            software_version,
            precision,
            interval_duration,
            unit,
            dimension,
            binary_representation,
            radar_locations,
            radar_location_contributions,
        })
    }
}

fn parse_produktkennung(input: &str) -> IResult<&str, Produktkennung> {
    let (input, produktkennung) = nom::branch::alt((tag("YW"), tag("RW"), tag("SF")))(input)?;
    Ok((
        input,
        match produktkennung {
            "YW" => Produktkennung::YW,
            "RW" => Produktkennung::RW,
            "SF" => Produktkennung::SF,
            _ => unreachable!(),
        },
    ))
}

fn parse_datetime_and_location(input: &str) -> IResult<&str, (PrimitiveDateTime, String)> {
    let (input, day) = take(2usize)(input)?;
    let (input, hour) = take(2usize)(input)?;
    let (input, minute) = take(2usize)(input)?;
    let (input, location) = take(5usize)(input)?;
    let (input, month) = take(2usize)(input)?;
    let (input, year) = take(2usize)(input)?;

    let datetime = &format!("20{year}-{month}-{day}T{hour}:{minute}");
    let datetime = PrimitiveDateTime::parse(&datetime, &Iso8601::DEFAULT).unwrap();
    Ok((input, (datetime, location.to_string())))
}

fn parse_product_length(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("BY")(input)?;
    let (input, product_length) = take(7usize)(input)?;
    let (input, product_length2) = digit0(input)?;
    Ok((
        input,
        format!("{}{}", product_length, product_length2)
            .parse::<u32>()
            .unwrap(),
    ))
}

fn parse_format_version(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("VS")(input)?;
    let (input, format_version) = take(2usize)(input)?;
    Ok((input, format_version.trim().to_string()))
}

fn parse_software_version(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("SW")(input)?;
    let (input, format_version) = take(9usize)(input)?;
    Ok((input, format_version.trim().to_string()))
}

fn parse_precision(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("PR")(input)?;
    let (input, precision) = take(5usize)(input)?;
    let precision = match precision.trim() {
        "E-00" => 1,
        "E-01" => 10,
        "E-02" => 100,
        _ => unreachable!(),
    };
    Ok((input, precision))
}

fn parse_interval_duration(input: &str) -> IResult<&str, u16> {
    let (input, _) = tag("INT")(input)?;
    let (input, interval_duration) = take(4usize)(input)?;
    let interval_duration = interval_duration.trim().parse::<u16>().unwrap();
    Ok((input, interval_duration))
}

fn parse_unit(input: &str) -> IResult<&str, Option<u8>> {
    let (input, unit) = opt(tag("U"))(input)?;
    if unit.is_none() {
        return Ok((input, None));
    }
    let (input, unit) = take(1usize)(input)?;
    Ok((input, unit.trim().parse::<u8>().ok()))
}

fn parse_dimension(input: &str) -> IResult<&str, Dimension> {
    let (input, _) = tag("GP")(input)?;
    let (input, rows) = take(4usize)(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, columns) = take(4usize)(input)?;

    Ok((
        input,
        Dimension {
            rows: rows.trim().parse::<u16>().unwrap(),
            columns: columns.trim().parse::<u16>().unwrap(),
        },
    ))
}

fn parse_binary_representation(input: &str) -> IResult<&str, Option<u32>> {
    let (input, br) = opt(tag("MF"))(input)?;
    if br.is_none() {
        return Ok((input, None));
    }
    let (input, br) = take(9usize)(input)?;
    Ok((input, br.trim().parse::<u32>().ok()))
}

fn parse_undocumented_vr(input: &str) -> IResult<&str, ()> {
    let (input, vr) = opt(tag("VR"))(input)?;
    if vr.is_none() {
        return Ok((input, ()));
    }
    let (input, _) = take(8usize)(input)?;
    Ok((input, ()))
}

fn parse_radar_locations(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = tag("MS")(input)?;
    let (input, length) = take(3usize)(input)?;
    let length = length.trim().parse::<usize>().unwrap();
    let (input, radar_locations) = take(length)(input)?;
    let radar_locations = radar_locations
        .trim()
        .strip_prefix("<")
        .unwrap()
        .strip_suffix(">")
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    Ok((input, radar_locations))
}

fn parse_radar_location_contributions(input: &str) -> IResult<&str, Option<Vec<(String, u8)>>> {
    let (input, st) = opt(tag("ST"))(input)?;
    if st.is_none() {
        return Ok((input, None));
    }
    let (input, length) = take(3usize)(input)?;
    let length = length.trim().parse::<usize>().unwrap();
    let (input, radar_locations) = take(length)(input)?;
    let radar_locations = radar_locations
        .trim()
        .strip_prefix("<")
        .unwrap()
        .strip_suffix(">")
        .unwrap()
        .split(',')
        .map(|s| {
            let mut s = s.split(' ');
            let radar_location = s.next().unwrap().to_string();
            let contribution = s.next().unwrap().trim().parse::<u8>().unwrap();
            (radar_location, contribution)
        })
        .collect::<Vec<(String, u8)>>();
    Ok((input, Some(radar_locations)))
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn yw() {
        let input = "YW010000100000117BY1980164VS 3SW   2.18.3PR E-02INT   5U0GP1100x 900MF 00000000VR2017.002MS 69<boo,ros,emd,hnr,umd,pro,ess,fld,drs,neu,nhb,oft,eis,tur,isn,fbg,mem>";
        let header = Header::from_str(input).unwrap();
        assert_eq!(
            header,
            Header {
                produktkennung: Produktkennung::YW,
                datetime: datetime!(2017-01-01 00:00:00),
                location: "10000".to_string(),
                product_length: 1980164,
                format_version: "3".to_string(),
                software_version: "2.18.3".to_string(),
                precision: 100,
                interval_duration: 5,
                unit: Some(0),
                dimension: Dimension {
                    rows: 1100,
                    columns: 900
                },
                binary_representation: Some(0),
                radar_locations: vec![
                    "boo".to_string(),
                    "ros".to_string(),
                    "emd".to_string(),
                    "hnr".to_string(),
                    "umd".to_string(),
                    "pro".to_string(),
                    "ess".to_string(),
                    "fld".to_string(),
                    "drs".to_string(),
                    "neu".to_string(),
                    "nhb".to_string(),
                    "oft".to_string(),
                    "eis".to_string(),
                    "tur".to_string(),
                    "isn".to_string(),
                    "fbg".to_string(),
                    "mem".to_string()
                ],
                radar_location_contributions: None
            }
        );
    }

    #[test]
    fn rw() {
        let input = "RW010050100000322BY1980160VS 3SW   2.18.3PR E-01INT  60U0GP1100x 900MF 00000001VR2017.002MS 65<boo,ros,hnr,umd,pro,ess,fld,drs,neu,nhb,oft,eis,tur,isn,fbg,mem>";
        let header = Header::from_str(input).unwrap();
        assert_eq!(
            header,
            Header {
                produktkennung: Produktkennung::RW,
                datetime: datetime!(2022-03-01 00:50:00),
                location: "10000".to_string(),
                product_length: 1980160,
                format_version: "3".to_string(),
                software_version: "2.18.3".to_string(),
                precision: 10,
                interval_duration: 60,
                unit: Some(0),
                dimension: Dimension {
                    rows: 1100,
                    columns: 900
                },
                binary_representation: Some(1),
                radar_locations: vec![
                    "boo".to_string(),
                    "ros".to_string(),
                    "hnr".to_string(),
                    "umd".to_string(),
                    "pro".to_string(),
                    "ess".to_string(),
                    "fld".to_string(),
                    "drs".to_string(),
                    "neu".to_string(),
                    "nhb".to_string(),
                    "oft".to_string(),
                    "eis".to_string(),
                    "tur".to_string(),
                    "isn".to_string(),
                    "fbg".to_string(),
                    "mem".to_string()
                ],
                radar_location_contributions: None
            }
        );
    }

    #[test]
    fn sf() {
        let input = "SF010050100000119BY1620267VS 3SW   2.21.0PR E-01INT1440GP 900x 900MS 70<asb,boo,ros,hnr,umd,pro,ess,fld,drs,neu,nhb,oft,eis,tur,isn,fbg,mem> ST120<asb 24,boo 24,drs 24,eis 24,ess 24,fbg 24,fld 24,hnr 24,isn 24,mem 24,neu 24,nhb 24,oft 24,pro 24,ros 24,tur 24,umd 24>";
        let header = Header::from_str(input).unwrap();
        assert_eq!(
            header,
            Header {
                produktkennung: Produktkennung::SF,
                datetime: datetime!(2019-01-01 00:50:00),
                location: "10000".to_string(),
                product_length: 1620267,
                format_version: "3".to_string(),
                software_version: "2.21.0".to_string(),
                precision: 10,
                interval_duration: 1440,
                unit: None,
                dimension: Dimension {
                    rows: 900,
                    columns: 900
                },
                binary_representation: None,
                radar_locations: vec![
                    "asb".to_string(),
                    "boo".to_string(),
                    "ros".to_string(),
                    "hnr".to_string(),
                    "umd".to_string(),
                    "pro".to_string(),
                    "ess".to_string(),
                    "fld".to_string(),
                    "drs".to_string(),
                    "neu".to_string(),
                    "nhb".to_string(),
                    "oft".to_string(),
                    "eis".to_string(),
                    "tur".to_string(),
                    "isn".to_string(),
                    "fbg".to_string(),
                    "mem".to_string()
                ],
                radar_location_contributions: Some(vec![
                    ("asb".to_string(), 24),
                    ("boo".to_string(), 24),
                    ("drs".to_string(), 24),
                    ("eis".to_string(), 24),
                    ("ess".to_string(), 24),
                    ("fbg".to_string(), 24),
                    ("fld".to_string(), 24),
                    ("hnr".to_string(), 24),
                    ("isn".to_string(), 24),
                    ("mem".to_string(), 24),
                    ("neu".to_string(), 24),
                    ("nhb".to_string(), 24),
                    ("oft".to_string(), 24),
                    ("pro".to_string(), 24),
                    ("ros".to_string(), 24),
                    ("tur".to_string(), 24),
                    ("umd".to_string(), 24)
                ])
            }
        );
    }

    #[test]
    fn test_error() {
        let data = [0b1100_0100, 0b0010_1001];
        let res = parse(&data);
        assert_eq!(res, BType::Error(2500));
    }

    #[test]
    fn test_parse_file() {
        let mut grid = Vec::new();
        (1..=18).for_each(|n| grid.push(n));
        let idx = offset_bottom_left_1_1(1, 1, 3, 6);
        assert_eq!(grid[idx], 13);
        let idx = offset_bottom_left_1_1(6, 3, 3, 6);
        assert_eq!(grid[idx], 6);
        let idx = offset_bottom_left_1_1(1, 2, 3, 6);
        assert_eq!(grid[idx], 7);
    }
}
