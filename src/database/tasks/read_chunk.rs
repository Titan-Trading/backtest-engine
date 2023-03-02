use std::sync::{Arc};
use crossbeam::channel::Sender;
use crate::database::{models::{candlestick::Candlestick}, storage::Reader};
use super::Task;


// a task that will read a chunk of data from a file
pub struct ReadChunkTask {
    pub channel: Arc<Sender<Vec<Candlestick>>>,
    pub filename: String,
    pub reader: Reader,
    pub limit: i32,
    pub offset: i32,
}
impl ReadChunkTask {
    // create a new read chunk task
    pub fn new(channel: Arc<Sender<Vec<Candlestick>>>, filename: String, reader: Reader, limit: i32, offset: i32) -> Self {
        Self {
            channel,
            filename,
            reader,
            limit,
            offset,
        }
    }
}

impl Task for ReadChunkTask {
    // execute the read chunk task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        // println!("thread.tasks.read_chunk: reading chunk of file {}", self.filename);

        // read the chunk of data from the file
        let bars: Vec<Candlestick> = match self.reader.read_chunk(self.limit, self.offset) {
            Ok(chunk) => chunk.candlesticks,
            Err(e) => {
                println!("thread.tasks.read_chunk: error reading chunk of file {}: {}", self.filename, e);
                return;
            }
        };

        // send the chunk of bars back to the query thread
        match self.channel.send(bars) {
            Ok(_) => {
                // println!("thread.tasks.read_chunk: chunk of bars sent to query thread");
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
}