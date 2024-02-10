use time::PrimitiveDateTime;
use time_tz::{timezones, OffsetDateTimeExt};

pub fn convert_utc_to_berlin(datetime: PrimitiveDateTime) -> PrimitiveDateTime {
    let berlin = timezones::db::europe::BERLIN;
    let dt = datetime.assume_utc();
    let converted = dt.to_timezone(berlin);
    PrimitiveDateTime::new(converted.date(), converted.time())
}

#[cfg(test)]
mod test {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test_convert_utc_to_berlin() {
        // MEZ
        let datetime = datetime!(2024-02-08 13:06:32);
        let converted = convert_utc_to_berlin(datetime);
        assert_eq!(converted, datetime!(2024-02-08 14:06:32));

        // MESZ
        let datetime = datetime!(2024-03-31 02:00:00);
        let converted = convert_utc_to_berlin(datetime);
        assert_eq!(converted, datetime!(2024-03-31 4:00:00));
    }
}
