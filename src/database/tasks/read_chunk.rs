use std::{sync::{Arc}, io::{Seek, SeekFrom}, fs::File};
use crossbeam::channel::Sender;
use crate::database::{models::{candlestick::Candlestick}, storage::reader::Reader};
use super::Task;


// a task that will read a chunk of data from a file
pub struct ReadChunkTask {
    pub channel: Arc<Sender<Vec<Candlestick>>>,
    pub filename: String,
    pub file_handle: File,
    pub limit: i64,
    pub offset: i64,
}
impl ReadChunkTask {
    // create a new read chunk task
    pub fn new(channel: Arc<Sender<Vec<Candlestick>>>, filename: String, file_handle: File, limit: i64, offset: i64) -> Self {
        Self {
            channel,
            filename,
            file_handle,
            limit,
            offset,
        }
    }
}

impl Task for ReadChunkTask {
    // execute the read chunk task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        
        // seek to the offset one time for each chunk
        let bytes_per_record = 54u64; // could be optimized
        let byte_offset = match self.file_handle.seek(SeekFrom::Start(self.offset as u64 * bytes_per_record)) {
            Ok(offset) => offset,
            Err(e) => {
                panic!("failed to seek to offset: {}", e);
            }
        };

        println!("byte offset: {}", byte_offset);

        // create a reader for the file
        let file_handle = self.file_handle.try_clone().unwrap();
        let mut reader = Reader::new_with_file(file_handle);

        // read the chunk of data from the file
        let bars: Vec<Candlestick> = match reader.read_chunk(self.limit) {
            Ok(chunk) => chunk.candlesticks,
            Err(e) => {
                println!("thread.tasks.read_chunk: error reading chunk of file {}: {}", self.filename, e);
                return;
            }
        };

        // send the chunk of bars back to the query thread
        match self.channel.send(bars) {
            Ok(_) => {
                println!("thread.tasks.read_chunk: chunk of bars sent to query thread");
            },
            Err(e) => {
                println!("thread.tasks.read_chunk: error sending chunk of bars to query thread: {}", e);
            }
        }

        // call the on exit callback
        if let Some(callback) = on_exit {
            callback(true);
        }
    }

    // close the file handle
    fn close(&mut self) {
        match self.file_handle.sync_all() {
            Ok(_) => {
                println!("thread.tasks.read_chunk: file handle closed");
            },
            Err(e) => {
                println!("thread.tasks.read_chunk: error closing file handle: {}", e);
            }
        }
    }
}