
// DatabaseEngine
// read data from the database
// manage read/write locks
// manage multi-threading reading from data files

use std::{io::Error, collections::HashMap, sync::{Arc, Mutex}};
use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::database::{tasks::query::QueryTask, threads::Task};

use super::{query::{Query, QueryResult}, database::Bar, cache::InMemoryCache, storage::engine::StorageEngine, consolidation::engine::ConsolidationEngine, threads::ThreadPool, synchronization::engine::SynchronizationEngine};

// database engine type
// defines the storage engine/corpus engine to use
#[derive(Clone)]
pub enum DatabaseEngineType {
    Csv,
    Stmdb,
}

// database engine
// sets the current working directory for the data files
// manages the storage engine, consolidation engine and cache
pub struct DatabaseEngine {
    pub path: String,
    thread_pool: Arc<ThreadPool>,
    storage_engine: Arc<Mutex<StorageEngine>>,
    synchronization_engine: Arc<Mutex<SynchronizationEngine>>,
    consolidation_engine: Arc<Mutex<ConsolidationEngine>>,
    cache: InMemoryCache,
}

impl DatabaseEngine {
    
    // creates a new database engine
    pub fn new(engine_type: DatabaseEngineType) -> DatabaseEngine {
        // create a thread pool for the database engine
        let thread_pool = Arc::new(ThreadPool::new(4));

        // the engine that stores and retrieves data from disk
        let storage_engine = Arc::new(Mutex::new(StorageEngine::new_with(engine_type, thread_pool.clone())));

        // the engine that synchronizes data from the storage engine
        let synchronization_engine = Arc::new(Mutex::new(SynchronizationEngine::new_with(thread_pool.clone())));

        // the engine that consolidates data from the storage engine
        let consolidation_engine = Arc::new(Mutex::new(ConsolidationEngine::new_with(thread_pool.clone())));

        // the cache that stores a queue of synchronized and consolidated data to be passed to the client
        let cache = InMemoryCache::new();

        DatabaseEngine {
            path: "./data".to_string(),
            thread_pool,
            storage_engine,
            synchronization_engine,
            consolidation_engine,
            cache,
        }
    }

    // create a new database engine with a given path
    pub fn new_with(engine_type: DatabaseEngineType, path: String) -> DatabaseEngine {
        // create a thread pool for the database engine
        let thread_pool = Arc::new(ThreadPool::new(4));

        // the engine that stores and retrieves data from disk
        let storage_engine = Arc::new(Mutex::new(StorageEngine::new_with(engine_type, thread_pool.clone())));

        // the engine that synchronizes data from the storage engine
        let synchronization_engine = Arc::new(Mutex::new(SynchronizationEngine::new_with(thread_pool.clone())));

        // the engine that consolidates data from the synchronization engine
        let consolidation_engine = Arc::new(Mutex::new(ConsolidationEngine::new_with(thread_pool.clone())));

        // the cache that stores a queue of synchronized and consolidated data to be passed to the client
        let cache = InMemoryCache::new();
        
        DatabaseEngine {
            path,
            thread_pool,
            storage_engine,
            synchronization_engine,
            consolidation_engine,
            cache,
        }
    }

    // query bars from the database
    pub fn query(&mut self, client_id: u64, query: Query, parameters: HashMap<String, String>) -> Result<QueryResult, Error> {

        // generate a unique id for the query using the client_id and query
        let query_id = query.to_hash(client_id).unwrap();
        println!("query_id: {}", query_id);

        // check cache for data (cache is read once, write many)
        if let Some(cached_results) = self.cache.get(&query_id.clone()) {
            return Ok(cached_results);
        }

        // create a new channel to send data to the client
        // wrap the channel in an Arc to share it between threads
        let (tx, rx): (Sender<QueryResult>, Receiver<QueryResult>) = unbounded();
        let tx = Arc::new(Mutex::new(tx));

        // create a new query task
        let mut task = QueryTask::new(Arc::clone(&tx), self.storage_engine.clone(), self.synchronization_engine.clone(), self.consolidation_engine.clone(), query.clone(), parameters.clone());

        // pass task to thread pool to be executed
        self.thread_pool.execute(move || {
            task.execute(Some(Box::new(|result: bool| {
                println!("query.complete result: {:?}", result);
            })));
        });

        // wait for the query task to complete
        let results = rx.recv().unwrap();

        // match query to know which data to load from disk
        // - limit
        // - start_time
        // - end_time
        // - intervals
        // - symbols
        // - exchanges

        // synchronize data before returning it

        // return data

        Ok(results)
    }

    // create a stream of live updates when new bars are inserted into the database
    pub fn stream<F>(&self, client_id: u64, query: Query, on_update: F) -> Result<bool, Error>
    where
        F: Fn(Query) -> Result<QueryResult, Error>,{

        // store link between query and on_update function for calling later

        Ok(true)
    }

    // insert bars into the database
    pub fn insert(&self, client_id: u64, data: Vec<Bar>) -> bool {

        // write data to disk using whatever storage engine we're using (buffered writes)
        // update cache with new data
        // call on_update functions for any queries that match the new data

        true
    }
}