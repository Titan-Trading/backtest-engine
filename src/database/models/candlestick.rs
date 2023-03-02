use std::fs::File;
use std::hash::{Hasher, Hash};
use std::io::{Error, BufReader};
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use super::field::{Field, FieldType};


// represents a single bar off a chart for a single symbol
#[derive(Debug, Clone, PartialEq)]
pub struct Candlestick {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Candlestick {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }

    pub fn new_with(timestamp: i64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        Self {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, Error> {
        // detect the end of a file
        let field_count_result = reader.read_u8(); // 1 byte
        if field_count_result.is_err() {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, "unexpected end of file"));
        }

        // read the number of fields from u8 - 6 fields
        let field_count = field_count_result.unwrap();

        // read the fields
        let mut fields = vec![];
        for i in 0..field_count {

            // read the length of the field
            let length = reader.read_u8().unwrap(); // 1 byte
            // println!("length: {}", length);

            // read the field type
            let field_type = reader.read_u8().unwrap(); // 1 byte
            // println!("field_type: {}", field_type);

            // read the original value
            let original_value;
            if field_type == 1 {
                // read as i64
                original_value = reader.read_i64::<BigEndian>().unwrap() as f64; // 8 bytes
            }
            else if field_type == 2 {
                // read as f64
                original_value = reader.read_f64::<BigEndian>().unwrap(); // 8 bytes
            }
            else {
                panic!("invalid field type");
            }

            // create a new field
            let field = Field {
                length,
                field_type: match field_type {
                    1 => FieldType::Int64,
                    2 => FieldType::Float64,
                    // 2 => FieldType::String,
                    _ => panic!("invalid field type"),
                },
                original_value,
            };

            // add the field to the list
            fields.push(field);
        }

        // println!("fields: {:?}", fields[0].original_value as i32);

        Ok(Self {
            timestamp: fields[0].original_value as i64,
            open: fields[1].original_value,
            high: fields[2].original_value,
            low:  fields[3].original_value,
            close: fields[4].original_value,
            volume: fields[5].original_value,
        })
    }

    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    pub fn set_open(&mut self, open: f64) {
        self.open = open;
    }

    pub fn set_high(&mut self, high: f64) {
        self.high = high;
    }

    pub fn set_low(&mut self, low: f64) {
        self.low = low;
    }

    pub fn set_close(&mut self, close: f64) {
        self.close = close;
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }
}

impl Hash for Candlestick {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.open.to_bits().hash(state);
        self.high.to_bits().hash(state);
        self.low.to_bits().hash(state);
        self.close.to_bits().hash(state);
        self.timestamp.hash(state);
    }
}

impl Eq for Candlestick {}