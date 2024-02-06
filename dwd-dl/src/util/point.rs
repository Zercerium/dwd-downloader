#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub column: T, //x
    pub row: T,    //y
}

impl<T> Point<T> {
    pub fn new(column: T, row: T) -> Self {
        Self { column, row }
    }
}

impl TryFrom<&str> for Point<u16> {
    type Error = ();

    /// first row, then column
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (row, column) = s.split_once(',').ok_or(())?;

        let row = row.parse::<u16>().map_err(|_| ())?;
        let column = column.parse::<u16>().map_err(|_| ())?;

        Ok(Self { row, column })
    }
}
