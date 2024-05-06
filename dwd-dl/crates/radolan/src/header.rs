use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{map_res, opt},
    error::{Error, ErrorKind},
    Err, Finish, IResult,
};
use thiserror::Error;
use time::{format_description::well_known::Iso8601, PrimitiveDateTime};

#[derive(PartialEq, Debug)]
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

impl Header {
    pub fn new(s: &str) -> Result<Self, HeaderParseError> {
        let (_, header) = parse_header(s)
            .finish()
            .map_err(|err| HeaderParseError::ParseError(err.to_string()))?;
        Ok(header)
    }
}

#[derive(Error, Debug)]
pub enum HeaderParseError {
    #[error("Parsing error in the input")]
    ParseError(String),
}

#[derive(PartialEq, Debug)]
pub enum Produktkennung {
    YW,
    RW,
    SF,
    NotTested(String),
}

#[derive(Debug, PartialEq)]
pub struct Dimension {
    pub rows: u16,
    pub columns: u16,
}

fn parse_header(s: &str) -> IResult<&str, Header> {
    let (s, produktkennung) = parse_produktkennung(s)?;
    let (s, (datetime, location)) = parse_datetime_and_location(s)?;
    let (s, product_length) = parse_product_length(s)?;
    let (s, format_version) = parse_format_version(s)?;
    let (s, software_version) = parse_software_version(s)?;
    let (s, precision) = parse_precision(s)?;
    let (s, interval_duration) = parse_interval_duration(s)?;
    let (s, unit) = opt(parse_unit)(s)?;
    let (s, dimension) = parse_dimension(s)?;
    let (s, binary_representation) = opt(parse_binary_representation)(s)?;
    let (s, _) = opt(parse_undocumented_vr)(s)?;
    let (s, radar_locations) = parse_radar_locations(s)?;
    let (s, radar_location_contributions) = opt(parse_radar_location_contributions)(s)?;
    s.is_empty()
        .then(|| ())
        .ok_or(Err::Error(Error::new("Input not Empty", ErrorKind::Fail)))?;

    Ok((
        s,
        Header {
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
        },
    ))
}

fn parse_produktkennung(input: &str) -> IResult<&str, Produktkennung> {
    let (input, produktkennung) = nom::branch::alt((tag("YW"), tag("RW"), tag("SF")))(input)?;
    Ok((
        input,
        match produktkennung {
            "YW" => Produktkennung::YW,
            "RW" => Produktkennung::RW,
            "SF" => Produktkennung::SF,
            s => Produktkennung::NotTested(s.to_string()),
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
    let datetime = PrimitiveDateTime::parse(datetime, &Iso8601::DEFAULT)
        // TODO how to keep the parse error
        .map_err(|_err| Err::Error(Error::new("Couldn't parse Datetime", ErrorKind::Fail)))?;
    Ok((input, (datetime, location.to_string())))
}

fn parse_product_length(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("BY")(input)?;
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
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
    map_res(take(4usize), |s: &str| s.trim().parse::<u16>())(input)
}

fn parse_unit(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("U")(input)?;
    map_res(take(1usize), |s: &str| s.trim().parse::<u8>())(input)
}

fn parse_dimension(input: &str) -> IResult<&str, Dimension> {
    let parse_u16 = |input| map_res(take(4usize), |s: &str| s.trim().parse::<u16>())(input);

    let (input, _) = tag("GP")(input)?;
    let (input, rows) = parse_u16(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, columns) = parse_u16(input)?;

    Ok((input, Dimension { rows, columns }))
}

fn parse_binary_representation(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("MF")(input)?;
    map_res(take(9usize), |s: &str| s.trim().parse::<u32>())(input)
}

fn parse_undocumented_vr(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("VR")(input)?;
    let (input, _) = take(8usize)(input)?;
    Ok((input, ()))
}

fn parse_radar_locations(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = tag("MS")(input)?;
    let (input, length) = map_res(take(3usize), |s: &str| s.trim().parse::<u16>())(input)?;

    let parse_radar_locations = |s: &str| {
        s.trim()
            .strip_prefix('<')
            .ok_or(())?
            .strip_suffix('>')
            .ok_or(())?
            .split(',')
            .map(|s| Ok(s.to_string()))
            .collect::<Result<Vec<_>, ()>>()
    };

    map_res(take(length), parse_radar_locations)(input)
}

fn parse_radar_location_contributions(input: &str) -> IResult<&str, Vec<(String, u8)>> {
    let (input, _) = tag("ST")(input)?;
    let (input, length) = map_res(take(3usize), |s: &str| s.trim().parse::<usize>())(input)?;

    let parse_radar_locations = |s: &str| {
        s.trim()
            .strip_prefix('<')
            .ok_or(())?
            .strip_suffix('>')
            .ok_or(())?
            .split(',')
            .map(|s| {
                let (radar_location, contribution) = s.split_once(' ').ok_or(())?;
                let contribution = contribution.trim().parse::<u8>().map_err(|_| ())?;
                Ok((radar_location.to_string(), contribution))
            })
            .collect::<Result<Vec<_>, ()>>()
    };

    map_res(take(length), parse_radar_locations)(input)
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn yw() {
        let input = "YW010000100000117BY1980164VS 3SW   2.18.3PR E-02INT   5U0GP1100x 900MF 00000000VR2017.002MS 69<boo,ros,emd,hnr,umd,pro,ess,fld,drs,neu,nhb,oft,eis,tur,isn,fbg,mem>";
        let header = Header::new(input).unwrap();
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
        let header = Header::new(input).unwrap();
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
        let header = Header::new(input).unwrap();
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
}
