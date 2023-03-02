use std::collections::HashMap;

use crate::{filesystem::types::stmdb::STMDBRecord, database::database::{BarSet, Bar}};


// synchronizing of data based on record timestamp from the storage engine across files
pub struct Synchronizer {
    pub files: Vec<String>,
    pub data: HashMap<String, Vec<STMDBRecord>>,
}

impl Synchronizer {
    
    // creates a new synchronizer
    pub fn new(files: Vec<String>, data: HashMap<String, Vec<STMDBRecord>>) -> Synchronizer {
        Synchronizer {
            files,
            data,
        }
    }

    // synchronizes data from the storage engine across files
    pub fn sync(&mut self, timestamp: i64) -> BarSet {

        // create a new bar set
        let mut bar_set = BarSet::new();

        // println!("synchronizer: syncing data for timestamp: {}", timestamp);
        // println!("synchronizer: from {} files", self.files.len());

        // iterate through each file
        for file in self.files.iter() {

            // get the data for the file
            let data = self.data.get(file).unwrap();

            // println!("synchronizer: file: {}", file);
            // println!("synchronizer: data length: {}", data.len());

            // parse exchange and symbol name from the file name
            let parts = file.split("_").collect::<Vec<&str>>();
            let exchange = parts[0];
            let symbol_name = parts[1];

            // iterate through each record
            for record in data.iter() {
                // if the record timestamp is equal to the timestamp we're looking for
                if record.timestamp == timestamp {
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
                }
            }
        }

        // println!("synchronizer: bar set length: {}", bar_set.bars.len());

        // return the bar set
        bar_set
    }
}