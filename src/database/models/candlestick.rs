use std::fs::File;
use std::hash::{Hasher, Hash};
use std::io::{Error,Write};
use byteorder::BigEndian;

use super::header::RECORD_SIZE;


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

    // create a candlestick using a byte array buffer from the file
    pub fn from_buffer(buffer: &mut Vec<u8>) -> Result<Self, Error> {
        // define how big each field is
        let offset_bytes = 9;

        // println!("buffer: {:?}", buffer);
        
        // read the fields
        let mut fields = vec![];
        for i in 0..6 {
            let buffer_index = if i == 0 { i } else { i * offset_bytes };

            // println!("buffer index: {}", buffer_index);

            // read the field type
            let field_type = buffer[buffer_index as usize]; // 1 byte

            // println!("field type: {}", field_type);

            // read the original value
            let mut original_value = 0.0f64;
            if field_type == 1u8 {
                // read as i64  - 8 bytes
                let mut i_buf: [u8; 8] = [0; 8];
                i_buf.copy_from_slice(&buffer[buffer_index+1..buffer_index+9]);
                let value = i64::from_be_bytes(i_buf);
                original_value = value as f64;
            }
            else if field_type == 2u8 {
                // read as f64 - 8 bytes
                let mut f_buf: [u8; 8] = [0; 8];
                f_buf.copy_from_slice(&buffer[buffer_index+1..buffer_index+9]);
                original_value = f64::from_be_bytes(f_buf);
            }
            else {
                panic!("invalid field type: {}", field_type);
            }

            // println!("original value: {}", original_value);

            // add the field to the list
            fields.push(original_value);
        }

        Ok(Self {
            timestamp: fields[0].round() as i64,
            open: fields[1],
            high: fields[2],
            low: fields[3],
            close: fields[4],
            volume: fields[5],
        })
    }

    pub fn into_writer(&mut self, file_instance: &mut File) -> Result<bool, Error> {
        // create buffer for header
        let mut buffer = [0; RECORD_SIZE];

        // write timestamp (1 type byte, 8 data bytes)
        buffer[0] = 1u8; // file type
        buffer[1..9].copy_from_slice(&self.timestamp.to_be_bytes()); // 8 bytes

        // write open (1 type byte, 8 data bytes)
        buffer[9] = 2u8; // field type
        buffer[10..18].copy_from_slice(&self.open.to_be_bytes()); // 8 bytes

        // write high (1 type byte, 8 data bytes)
        buffer[18] = 2u8; // field type
        buffer[19..27].copy_from_slice(&self.high.to_be_bytes()); // 8 bytes

        // write low (1 type byte, 8 data bytes)
        buffer[27] = 2u8; // field type
        buffer[28..36].copy_from_slice(&self.low.to_be_bytes()); // 8 bytes

        // write close (1 type byte, 8 data bytes)
        buffer[36] = 2u8; // field type
        buffer[37..45].copy_from_slice(&self.close.to_be_bytes()); // 8 bytes

        // write volume (1 type byte, 8 data bytes)
        buffer[45] = 2u8; // field type
        buffer[46..54].copy_from_slice(&self.volume.to_be_bytes()); // 8 bytes

        // write record buffer into file
        let result = file_instance.write(&mut buffer);

        // check if the write was successful
        let bytes_written = match result {
            Ok(bytes_written) => bytes_written,
            Err(e) => return Err(e),
        };

        // flush the file
        file_instance.flush()?;

        Ok(bytes_written == RECORD_SIZE)
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