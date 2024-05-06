#[derive(Debug)]
pub enum Record {
    Normal(u16),       // 0000
    Interpolated(u16), // 0001 ;Hail if Produkt: RE
    Error(u16),        // 0010
    Neg(u16),          // 0100
    Scope(u16),        // 1000
}

impl Record {
    pub(super) fn parse(bytes: &[u8]) -> Result<Self, (u8, u16)> {
        let data_type = bytes[1] >> 4;
        let value = ((bytes[1] & 0b0000_1111) as u16) << 8 | bytes[0] as u16;
        let record = match data_type {
            0b0000 => Record::Normal(value),
            0b0001 => Record::Interpolated(value),
            0b0010 => Record::Error(value),
            0b0100 => Record::Neg(value),
            0b1000 => Record::Scope(value),
            _ => return Err((data_type, value)),
        };
        Ok(record)
    }

    pub fn default_f32(&self, precision: u8) -> f32 {
        let precision = precision as f32;
        match self {
            Record::Normal(v) => *v as f32 / precision,
            Record::Interpolated(v) => *v as f32 / precision,
            Record::Error(_) => -9_f32,
            Record::Neg(v) => -(*v as f32 / precision),
            Record::Scope(v) => *v as f32 / precision,
        }
    }
}
