use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, Arc};
use crossbeam::channel::Sender;

use crate::filesystem::reader::ReaderResult;
use crate::filesystem::types::stmdb::STMDBRecord;
use crate::{database::database::Bar, filesystem::reader::Reader};
use crate::database::threads::Task;


// a job that performs a one or multiple writes to a file
pub struct WriteTask {
    file_name: String,
    records: Vec<Bar>,
}
impl WriteTask {

    // create a new write file task
    pub fn new(file_name: String, records: Vec<Bar>) -> Self {
        Self {
            file_name,
            records,
        }
    }
}
impl Task for WriteTask {

    // execute the write file task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        println!("Writing to file: {}", self.file_name);
    }
}

// a job that performs a one or multiple reads from a file
pub struct ReadTask {
    reader: Reader,
    channel: Arc<Mutex<Sender<Vec<STMDBRecord>>>>,
    filename: String,
    parameters: HashMap<String, String>,
}
impl ReadTask {

    // create a new read file task
    pub fn new(reader: Reader, channel: Arc<Mutex<Sender<Vec<STMDBRecord>>>>, filename: String, parameters: &HashMap<String, String>) -> Self {
        Self {
            reader,
            channel,
            filename,
            parameters: parameters.clone(),
        }
    }
}
impl Task for ReadTask {

    // execute the read file task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        let parameters = self.parameters.clone();

        let results = self.reader.read_file(self.filename.clone(), parameters);
        if let Err(error) = results {
            println!("thread.tasks.files: failed to read file: {}", error);

            // call the on exit callback
            if let Some(callback) = on_exit {
                callback(false);
            }
        }
        else {
            println!("thread.tasks.files: read file: {}", self.filename);

            // try to get the lock on the channel
            // if we can't get the lock, then we need to wait until we can get the lock
            let channel = match self.channel.try_lock() {
                Ok(channel) => {
                    println!("thread.tasks.files: got lock on channel");
                    channel
                },
                Err(_) => {
                    panic!("thread.tasks.files: failed to get lock on channel");
                }
            };

            let reader_results = results.unwrap();
            match reader_results {
                ReaderResult::STMDBRecords(results) => {
                    println!("thread.tasks.files: sending results to channel");
                    
                    // send the results to the channel
                    if let Err(error) = channel.send(results) {
                        println!("thread.tasks.files: failed to send results to channel: {}", error);
                    }
                    println!("thread.tasks.files: sent results to channel");

                    // call the on exit callback
                    if let Some(callback) = on_exit {
                        callback(true);
                    }
                },
                _ => {
                    println!("thread.tasks.files: no results to send to channel");
                }
            }
        }
    }
}