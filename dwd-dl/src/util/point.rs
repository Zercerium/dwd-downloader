use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T, //x
    pub y: T, //y
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<U> FromStr for Point<U>
where
    U: std::str::FromStr,
{
    type Err = ();

    /// first row, then column
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;

        let x = x.parse::<U>().map_err(|_| ())?;
        let y = y.parse::<U>().map_err(|_| ())?;

        Ok(Self { x, y })
    }
}
