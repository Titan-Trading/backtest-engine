use std::{sync::{Arc, Mutex}, collections::HashMap, time::Instant, fs::File};
use crossbeam::channel::Sender;

use crate::database::{models::{barset::BarSet, candlestick::Candlestick, bar::Bar}, tasks::read_chunk::ReadChunkTask, threads::ThreadPool};
use super::Task;


// query task - starts the tasks for reading file chunks, synchronizing the data, and consolidating the data
pub struct QueryTask {
    stop_flag: bool,
    thread_pool: Arc<ThreadPool>,
    channel: Arc<Mutex<Sender<BarSet>>>,
    files: Vec<String>,
    limit: i32,
    start_timestamp: i64,
    end_timestamp: i64,
}

impl QueryTask {
    // create a new query task
    pub fn new(
        thread_pool: Arc<ThreadPool>,
        channel: Arc<Mutex<Sender<BarSet>>>,
        files: Vec<String>,
        start_timestamp: i64,
        end_timestamp: i64,
        limit: i32
    ) -> Self {
        Self {
            stop_flag: false,
            thread_pool,
            channel,
            files,
            start_timestamp,
            end_timestamp,
            limit
        }
    }
}

impl Task for QueryTask {
    // execute the query task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        println!("thread.tasks.query: querying files");
        println!("thread.tasks.query: files: {:?}", self.files);

        // define pagination
        let total_bars = (self.end_timestamp - self.start_timestamp) / 60;
        let page_size = self.limit as i64;
        let mut page_count = total_bars / page_size;
        if total_bars % page_size != 0 {
            page_count += 1;
        }

        // create a channel for each page to send the bars back to the query task
        let mut receivers = HashMap::new();

        println!("thread.tasks.query: start timestamp: {}", self.start_timestamp);
        println!("thread.tasks.query: end timestamp: {}", self.end_timestamp);
        println!("thread.tasks.query: total bars: {}", total_bars);
        println!("thread.tasks.query: page size: {}", page_size);
        println!("thread.tasks.query: page count: {}", page_count);

        // loop through all the pages using the start_timestamp and the end_timestamp
        // setup all pages before waiting for results on any page
        // create a read chunk task for each file
        let mut readers = HashMap::new();

        // loop through all the files to create file descriptors before starting the read chunk tasks
        for filename in self.files.iter() {
            // create a new descriptor for the file
            let file = File::open(filename).unwrap();

            // add the file instance to the files map
            readers.insert(filename.to_string(), file.try_clone().unwrap());
        }

        for page in 0..page_count {
            println!("thread.tasks.query: page {}", page + 1);

            // check if stop flag has been set
            if self.stop_flag {
                println!("thread.tasks.query: stop flag set, stopping query");
                break;
            }

            // loop through all the files for the query
            for filename in self.files.iter() {
                let start = Instant::now();

                // get the reader for the file
                let reader = readers.get(filename).unwrap();

                // create a channel for the read chunk task to send the bars back to the query task
                let (sender, receiver) = crossbeam::channel::unbounded();
                receivers.insert(format!("{}{}", page, filename), receiver);
                let channel = Arc::new(sender);

                // create a new read chunk task
                let offset = page * page_size;
                let mut read_task = ReadChunkTask::new(Arc::clone(&channel), filename.clone(), reader.try_clone().unwrap(), page_size, offset);

                // start the read chunk task
                let filename_thread = filename.clone().to_string();

                // self.thread_pool.execute(move || {
                //     read_task.execute(None);
                // });
                
                // execute the read chunk task
                read_task.execute(Some(Box::new(move |result: bool| {
                    println!("thread.tasks.query: read chunk task {} finished in {}ms", filename_thread, (start.elapsed().as_nanos() as f64 / 1_000_000.0));
                })));
            }
        };
        

        for page in 0..page_count {
            let mut file_bars: HashMap<String, Vec<Candlestick>> = HashMap::new();

            // check if stop flag has been set
            if self.stop_flag {
                println!("thread.tasks.query: stop flag set, stopping query");
                break;
            }
            
            // loop through all files to get the read chunk tasks results
            for filename in self.files.iter() {
                // get the receiver for the read chunk task
                let receiver = match receivers.get(&format!("{}{}", page, filename)) {
                    Some(receiver) => receiver,
                    None => {
                        println!("thread.tasks.query: error getting receiver for file {}", filename);
                        return;
                    }
                };

                // wait for the read chunk task to finish
                let bars = match receiver.recv() {
                    Ok(bars) => bars,
                    Err(e) => {
                        println!("thread.tasks.query: error receiving bars from read chunk task: {}", e);
                        return;
                    }
                };

                // add the bars to the file_bars map
                file_bars.insert(filename.to_string(), bars);
            }

            // synchronize the bars from the files into a set of bars a barset
            let start = Instant::now();
            let mut barset = BarSet::new();
            for (filename, candlesticks) in file_bars.iter() {
                // println!("thread.tasks.query: candlesticks: {:?}", candlesticks);

                let parts: Vec<&str> = filename.split('/').collect();
                let exchange_symbol: Vec<&str> = parts[2].split('_').collect();
                let exchange = exchange_symbol[0];
                let symbol = exchange_symbol[1].replace(".stmdb", "");

                for candlestick in candlesticks.iter() {
                    let timestamp = candlestick.timestamp;

                    // check if the barset has a bar representing the timestamp
                    let bar = barset.bars.iter_mut().find(|b| b.timestamp == timestamp);
                    match bar {
                        Some(bar) => {
                            // has a bar for the timestamp
                            // check if we have the candlestick in that bar
                            let source_id = format!("{}:{}", exchange, symbol);
                            if !bar.has_candlestick(source_id) {
                                // add the candlestick to the bar
                                bar.add_candlestick(format!("{}:{}", exchange, symbol), candlestick.clone());
                            }
                        },
                        None => {
                            // does not have a bar for the timestamp
                            // create a new bar
                            let mut bar = Bar::new(timestamp);

                            // add the candlestick to the bar
                            bar.add_candlestick(format!("{}:{}", exchange, symbol), candlestick.clone());

                            // add the bar to the barset
                            barset.bars.push(bar);
                        }
                    }
                }
            }

            println!("thread.tasks.query: barset of {} bars synchronized in {}ms", barset.bars.len(), (start.elapsed().as_nanos() as f64 / 1_000_000.0));

            // last page of results for the query task
            if page == page_count - 1 {
                barset.is_last = true;
            }

            // send the barset back to the main thread
            match self.channel.lock().unwrap().send(barset) {
                Ok(_) => {
                    // println!("thread.tasks.query: barsets sent to main thread");
                },
                Err(e) => {
                    println!("thread.tasks.query: error sending barsets to main thread: {}", e);
                }
            }
        }

        // call the on exit function
        if let Some(on_exit) = on_exit {
            on_exit(true);
        }
    }

    fn close(&mut self) {
        self.stop_flag = true;
    }
}