use std::{collections::HashMap, sync::Arc};
use std::io::Error;
use crate::{database::{database::{BarSet, Bar}, threads::ThreadPool}};


// this engine will take the results from the storage engine and sync them up using the timestamp
pub struct SynchronizationEngine {
    pub thread_pool: Arc<ThreadPool>,
}

impl SynchronizationEngine {

    // creates a new synchronization engine
    pub fn new(thread_pool_size: i8) -> Self {
        let thread_pool = Arc::new(ThreadPool::new(thread_pool_size as usize));

        Self {
            thread_pool,
        }
    }

    // create a new synchronization engine with a given thread pool
    pub fn new_with(thread_pool: Arc<ThreadPool>) -> Self {
        Self {
            thread_pool,
        }
    }

    // synchronizes data from the storage engine across files
    pub fn sync(&mut self, files: Vec<String>, data: HashMap<String, Vec<Bar>>) -> Result<Vec<BarSet>, Error> {

        // create a new bar set
        let mut bar_sets = Vec::new();
        let mut bar_set = BarSet::new();

        // println!("synchronizer: syncing data for timestamp: {}", timestamp);
        // println!("synchronizer: from {} files", self.files.len());

        // iterate through each file
        for file in files.iter() {

            // get the data for the file
            let data = data.get(file).unwrap();

            // println!("synchronizer: file: {}", file);
            // println!("synchronizer: data length: {}", data.len());

            // parse exchange and symbol name from the file name
            let parts = file.split("_").collect::<Vec<&str>>();
            let exchange = parts[0];
            let symbol_name = parts[1];

            // iterate through each record
            for record in data.iter() {
                // if the record timestamp is equal to the timestamp we're looking for
                /*if record.timestamp == timestamp {
                    println!("synchronizer: found record for timestamp: {}", timestamp);

                    // create a new bar
                    let bar = Bar::new_with(
                        symbol_name.to_string(),
                        exchange.to_string(),
                        String::from("1m"),
                        record.open,
                        record.high,
                        record.low,
                        record.close,
                        record.volume,
                        timestamp
                    );

                    // add the bar to the bar set
                    bar_set.bars.insert(file.to_string(), bar);
                }*/
            }
        }

        // println!("synchronizer: bar set length: {}", bar_set.bars.len());

        // return the bar set
        Ok(bar_sets)
    }
}