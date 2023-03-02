use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use crossbeam::channel::Sender;

use crate::database::consolidation::engine::ConsolidationEngine;
use crate::database::query::{QueryResult, Query};
use crate::database::storage::corpus::CorpusIndex;
use crate::database::storage::engine::StorageEngine;
use crate::database::synchronization::engine::SynchronizationEngine;
use crate::database::threads::Task;


// a job that performs a query with the corpus and consolidation engines
pub struct QueryTask {
    channel: Arc<Mutex<Sender<QueryResult>>>,
    storage_engine: Arc<Mutex<StorageEngine>>,
    synchronization_engine: Arc<Mutex<SynchronizationEngine>>,
    consolidation_engine: Arc<Mutex<ConsolidationEngine>>,
    query: Query,
    parameters: HashMap<String, String>,
}
impl QueryTask {

    // create a new read file task
    pub fn new(
        channel: Arc<Mutex<Sender<QueryResult>>>,
        storage_engine: Arc<Mutex<StorageEngine>>,
        synchronization_engine: Arc<Mutex<SynchronizationEngine>>,
        consolidation_engine: Arc<Mutex<ConsolidationEngine>>,
        query: Query,
        parameters: HashMap<String, String>
    ) -> Self {
        Self {
            channel,
            storage_engine,
            synchronization_engine,
            consolidation_engine,
            query,
            parameters: parameters.clone(),
        }
    }
}

impl Task for QueryTask {

    // execute the read file task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        let parameters = self.parameters.clone();
        let query = self.query.clone();

        // try to get the lock on the storage engine
        // if we can't get the lock, then we need to wait until we can get the lock
        let mut storage_engine = match self.storage_engine.try_lock() {
            Ok(engine) => {
                println!("thread.tasks.query: lock on storage engine");
                engine
            },
            Err(_) => {
                panic!("thread.tasks.query: failed lock on storage engine");
            }
        };

        // if data is not in cache, read from disk using whatever storage engine we're using
        // returns a hashmap where the key is the filename and the value is a vector of bars/records
        let results = storage_engine.query(query.clone(), &parameters).unwrap();
        println!("thread.tasks.query: storage engine results: {:?}", results);

        // try to get the lock on the synchronization engine
        // if we can't get the lock, then we need to wait until we can get the lock
        let mut synchronization_engine = match self.synchronization_engine.try_lock() {
            Ok(engine) => {
                println!("thread.tasks.query: lock on synchronization engine");
                engine
            },
            Err(_) => {
                panic!("thread.tasks.query: failed lock on synchronization engine");
            }
        };

        // get the files that are in the corpus using the query
        let corpus_index = CorpusIndex::from_query("index.json".to_string(), storage_engine.filesystem.cwd.clone(), query.clone());
        let files: Vec<String> = corpus_index.corpus_map.iter().map(|(_, corpus)| corpus.filename.clone()).collect();
        
        // synchronize the results with the other files in the corpus
        // this will return a vector of barset records, barsets have all files compiled together
        let results = synchronization_engine.sync(files, results).unwrap();
        println!("thread.tasks.query: synchronization engine results: {:?}", results);

        // try to get the lock on the consolidation engine
        // if we can't get the lock, then we need to wait until we can get the lock
        let mut consolidation_engine = match self.consolidation_engine.try_lock() {
            Ok(engine) => {
                println!("thread.tasks.query: lock on consolidation engine");
                engine
            },
            Err(_) => {
                panic!("thread.tasks.query: failed lock on consolidation engine");
            }
        };

        // take synchronization engine results and pass them to the consolidate engine
        let results = consolidation_engine.consolidate(query.intervals.unwrap(), results).unwrap();
        println!("thread.tasks.query: consolidation engine results: {:?}", results);

        // try to get the lock on the channel
        // if we can't get the lock, then we need to wait until we can get the lock
        let channel = match self.channel.try_lock() {
            Ok(engine) => {
                println!("thread.tasks.query: lock on channel");
                engine
            },
            Err(_) => {
                panic!("thread.tasks.query: failed lock on channel");
            }
        };

        // create a new query result
        let results = QueryResult::new_with(results);
        println!("thread.tasks.query: query results: {:?}", results);

        // send the results to the channel
        match channel.send(results) {
            Ok(_) => {
                println!("thread.tasks.query: results sent to main thread");
            },
            Err(e) => {
                println!("thread.tasks.query: error sending query results to main thread: {}", e);
            }
        }

        // call the on exit function
        if let Some(on_exit) = on_exit {
            on_exit(true);
        }

        // take results and pass them to another thread to be processed
        // processing will include synchronization of records across threads/files using timestamp
        // processing will also include consolidation of records based on different intervals of time (1m, 5m, 15m, 1h, 1d, 1w, 1M, 1y, any other ones)
    
        // this thread pool is used to read files from disk
        // there will be another thread pool used to consolidate and synchronize records
    }
}