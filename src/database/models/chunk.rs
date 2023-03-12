use std::{io::{Read}, fs::File};
use std::io::Error;

use super::candlestick::Candlestick;

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
    pub fn from_file(file: &mut File, limit: i64) -> Result<Self, Error> {
        // create a new chunk
        let mut chunk: Vec<Candlestick> = Vec::new();

        // create a buffer from the file
        // each record is 54 bytes and each chunk is 1000 records then a chunk buffer is ~5.4kb 
        let mut chunk_buffer = vec![0;54 * limit as usize];
        let file = file;
        file.read(&mut chunk_buffer).unwrap();

        // read the chunk 0 to 999 records
        for record_index in 0..limit {

            // get a range of certain bytes from the buffer
            let record_start_index = record_index as usize * 54;
            let record_end_index = (record_index as usize + 1) * 54;

            // if the record is empty then break
            if chunk_buffer[record_start_index] == 0 {
                break;
            }

            // create a buffer for the candlestick
            let mut candle_buffer = chunk_buffer[record_start_index..record_end_index].to_vec();

            // read a candlestick from the reader
            match Candlestick::from_buffer(&mut candle_buffer) {
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