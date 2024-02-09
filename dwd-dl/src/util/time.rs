use time::{
    format_description::{well_known::iso8601, FormatItem},
    macros::format_description,
    Date, PrimitiveDateTime, Time,
};

pub mod timezone;

pub fn parse_yyyymm(s: &str) -> Result<Date, ()> {
    let year = s[..4].parse().unwrap();
    let month: u8 = s[4..].parse().unwrap();
    Date::from_calendar_date(year, month.try_into().unwrap(), 1).map_err(|_| ())
}

// TODO Error Handling
pub fn parse_yyyymmdd(s: &str) -> Result<Date, ()> {
    static FORMAT: &[FormatItem<'_>] = format_description!("[year][month][day]");
    Date::parse(s, &FORMAT).map_err(|_| ())
}

// TODO Error Handling
pub fn parse_yyyymmdd_into_date_time(s: &str) -> Result<PrimitiveDateTime, ()> {
    let date = parse_yyyymmdd(s)?;
    Ok(PrimitiveDateTime::new(date, Time::MIDNIGHT))
}

// TODO Error Handling
pub fn parse_yyyymmddhhmm(s: &str) -> Result<PrimitiveDateTime, ()> {
    let format = format_description!("[year][month][day][hour][minute]");
    PrimitiveDateTime::parse(s, &format).map_err(|_| ())
}

// TODO Error Handling
pub fn parse_yyyymmddhh(s: &str) -> Result<PrimitiveDateTime, ()> {
    let format = format_description!("[year][month][day][hour]");
    PrimitiveDateTime::parse(s, &format).map_err(|_| ())
}

pub fn format_yyyymmddhhmm(date: PrimitiveDateTime) -> String {
    let format = format_description!("[year][month][day][hour][minute]");
    date.format(format).unwrap()
}

/// hh:mm
pub fn format_time_colon(date: PrimitiveDateTime) -> String {
    let format = format_description!("[hour]:[minute]");
    date.format(format).unwrap()
}

/// MM/DD/JJJJ
pub fn format_date_american(date: PrimitiveDateTime) -> String {
    let format = format_description!("[month]/[day]/[year]");
    date.format(format).unwrap()
}

/// YYYY-MM-DD
pub fn format_date_iso(date: Date) -> String {
    date.format(&iso8601::Iso8601::DATE).unwrap()
}

/// YYYY-MM-DD
pub fn format_time_iso(time: Time) -> String {
    let format = format_description!("[hour]:[minute]");
    time.format(&format).unwrap()
}
