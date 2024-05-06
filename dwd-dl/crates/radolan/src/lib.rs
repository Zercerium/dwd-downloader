use std::str::Utf8Error;

use header::Header;
use record::Record;
use thiserror::Error;

pub mod header;
pub mod record;

const END_OF_TEXT: u8 = 0x03; // ETX

pub struct Radolan<'a> {
    data: &'a [u8],
    header_end: usize,
    header: Header,
}

impl<'a> Radolan<'a> {
    pub fn new(file: &'a [u8]) -> Result<Self, RadolanReadError> {
        let header_end = file
            .iter()
            .position(|b| b.eq(&END_OF_TEXT))
            .ok_or(RadolanReadError::EndOfTextMissing)?;

        let header_str = std::str::from_utf8(&file[..header_end])
            .map_err(|err| RadolanReadError::Utf8Error(err))?
            .to_string();

        // check if long enough

        Ok(Radolan {
            data: file,
            header_end,
            header: Header::new(header_str.as_str())
                .map_err(|err| RadolanReadError::HeaderParseError(err))?,
        })
    }

    pub fn get_point(&self, row: u16, column: u16) -> Result<Record, (u8, u16)> {
        if row >= self.header.dimension.rows {
            return Err((0, 0));
        }
        if column >= self.header.dimension.columns {
            return Err((0, 0));
        }
        let row = row as usize;
        let column = column as usize;
        // data set is beginning with the point at the bottom left, but we want to index from top left
        let row = self.header.dimension.rows as usize - 1 - row;
        let record_width = 2;
        let offset = (row * self.header.dimension.columns as usize + column) * record_width
            + self.header_end
            + 1;
        let offset_end = offset + record_width;

        let record = &self.data[offset..offset_end];
        Ok(Record::parse(record)?)
    }

    pub fn header(&self) -> &header::Header {
        &self.header
    }
}

#[derive(Error, Debug)]
pub enum RadolanReadError {
    #[error("`End of Text`(etx) missing")]
    EndOfTextMissing,
    #[error("Header is not Valid UTF-8")]
    Utf8Error(Utf8Error),
    #[error("Header is not Valid UTF-8")]
    HeaderParseError(header::HeaderParseError),
}
