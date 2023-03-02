use std::{io::{BufReader, ErrorKind}, fs::File};
use std::io::Error;
use super::{bar::Bar, candlestick::Candlestick, barset::BarSet};


pub struct Chunk {
    pub candlesticks: Vec<Candlestick>,
}

impl Chunk {
    // create a new chunk
    pub fn new() -> Self {
        Self {
            candlesticks: Vec::new(),
        }
    }

    // create a new chunk with a set of candlesticks
    pub fn new_with(candlesticks: Vec<Candlestick>) -> Self {
        Self {
            candlesticks,
        }
    }

    // read a chunk from a reader
    pub fn from_reader(reader: &mut BufReader<File>, limit: i32, offset: i32) -> Result<Self, Error> {
        // create a new chunk
        let mut chunk: Vec<Candlestick> = Vec::new();

        // seek to the offset
        let bytes_per_record = 61;
        match reader.seek_relative(offset as i64 * bytes_per_record as i64) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, format!("failed to seek to offset: {}", e)));
            }
        }

        // read the chunk
        for _ in 0..limit {
            // read a bar from the reader
            match Candlestick::from_reader(reader) {
                Ok(bar) => {
                    // add the bar to the chunk
                    chunk.push(bar);
                },
                Err(e) => {
                    break;
                }
            }
        }

        // return the chunk
        Ok(Self::new_with(chunk))
    }

    // add a candlestick to the chunk
    pub fn add_candlestick(&mut self, candlestick: Candlestick) {
        self.candlesticks.push(candlestick);
    }
}