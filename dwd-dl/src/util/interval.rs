use time::{macros::time, Date, PrimitiveDateTime, Time};

use super::time::{parse_yyyymmdd, parse_yyyymmddhhmm};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<T>
where
    T: std::cmp::PartialOrd,
{
    pub start: T,
    pub end: T,
}

impl<T> Interval<T>
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    pub fn new(start: T, end: T) -> Result<Self, ()> {
        if start > end {
            return Err(());
        }
        Ok(Self { start, end })
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }

    pub fn contains(&self, other: &T) -> bool {
        self.start <= *other && *other < self.end
    }
}

impl Interval<PrimitiveDateTime> {
    pub fn parse_str(start: &str, end: &str) -> Self {
        let parser = |s: &str| parse_yyyymmddhhmm(s).unwrap();
        let start = parser(start);
        let end = parser(end);
        Self::new(start, end).unwrap()
    }
}

impl Interval<Date> {
    pub fn parse_str(start: &str, end: &str) -> Self {
        let parser = |s: &str| parse_yyyymmdd(s).unwrap();
        let start = parser(start);
        let end = parser(end);
        Self::new(start, end).unwrap()
    }
}

pub trait Overlaps<Rhs = Self> {
    fn before(&self, other: &Rhs) -> bool;

    fn after(&self, other: &Rhs) -> bool;

    fn overlaps(&self, other: &Rhs) -> bool {
        !(self.before(other) || self.after(other))
    }
}

impl<T> Overlaps for Interval<T>
where
    T: std::cmp::PartialOrd,
{
    fn before(&self, other: &Self) -> bool {
        self.end < other.start
    }

    fn after(&self, other: &Self) -> bool {
        self.start > other.end
    }
}

impl Overlaps<Interval<Date>> for Interval<PrimitiveDateTime> {
    fn before(&self, other: &Interval<Date>) -> bool {
        self.end.date() < *other.start()
    }

    fn after(&self, other: &Interval<Date>) -> bool {
        self.start.date() > other.end
    }
}

impl Overlaps<Interval<PrimitiveDateTime>> for Interval<Date> {
    fn before(&self, other: &Interval<PrimitiveDateTime>) -> bool {
        self.end < other.start().date()
    }

    fn after(&self, other: &Interval<PrimitiveDateTime>) -> bool {
        self.start > other.end().date()
    }
}

impl From<Interval<Date>> for Interval<PrimitiveDateTime> {
    fn from(interval: Interval<Date>) -> Self {
        let start = PrimitiveDateTime::new(*interval.start(), Time::MIDNIGHT);
        let end = PrimitiveDateTime::new(*interval.end(), time!(23:59:59.999_999_999));
        Interval::new(start, end).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_contains() {
        let date = date!(2022 - 11 - 26);
        let interval = Interval::new(date!(2022 - 11 - 26), date!(2022 - 11 - 26)).unwrap();
        let res = interval.contains(&date);
        assert!(!res);

        let interval = Interval::new(date!(2022 - 11 - 26), date!(2022 - 11 - 27)).unwrap();
        let res = interval.contains(&date);
        assert!(res);
    }
}
