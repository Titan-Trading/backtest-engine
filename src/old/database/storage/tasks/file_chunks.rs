use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, Arc};
use crossbeam::channel::Sender;

use crate::filesystem::reader::ReaderResult;
use crate::filesystem::types::stmdb::STMDBRecord;
use crate::{database::database::Bar, filesystem::reader::Reader};
use crate::database::threads::Task;


// a job that performs a one or multiple reads from a file chunk
pub struct ReadChunkTask {
    reader: Reader,
    channel: Arc<Mutex<Sender<Vec<STMDBRecord>>>>,
    filename: String,
    parameters: HashMap<String, String>,
}
impl ReadChunkTask {

    // create a new read file chunk task
    pub fn new(reader: Reader, channel: Arc<Mutex<Sender<Vec<STMDBRecord>>>>, filename: String, parameters: &HashMap<String, String>) -> Self {
        Self {
            reader,
            channel,
            filename,
            parameters: parameters.clone(),
        }
    }
}
impl Task for ReadChunkTask {

    // execute the read file chunk task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        let parameters = self.parameters.clone();

        let results = self.reader.read_chunk(self.filename.clone(), parameters);
        if let Err(error) = results {
            println!("thread.tasks.file_chunks: failed to read file chunk: {}", error);

            // call the on exit callback
            if let Some(callback) = on_exit {
                callback(false);
            }
        }
        else {
            println!("thread.tasks.file_chunks: read file chunk: {}", self.filename);

            // try to get the lock on the channel
            // if we can't get the lock, then we need to wait until we can get the lock
            let channel = match self.channel.try_lock() {
                Ok(channel) => {
                    println!("thread.tasks.file_chunks: got lock on channel");
                    channel
                },
                Err(_) => {
                    panic!("thread.tasks.file_chunks: failed to get lock on channel");
                }
            };

            let reader_results = results.unwrap();
            match reader_results {
                ReaderResult::STMDBRecords(results) => {
                    println!("thread.tasks.file_chunks: sending results to channel");
                    
                    // send the results to the channel
                    if let Err(error) = channel.send(results) {
                        println!("thread.tasks.file_chunks: failed to send results to channel: {}", error);
                    }
                    println!("thread.tasks.file_chunks: sent results to channel");

                    // call the on exit callback
                    if let Some(callback) = on_exit {
                        callback(true);
                    }
                },
                _ => {
                    println!("thread.tasks.file_chunks: no results to send to channel");
                }
            }
        }
    }
}