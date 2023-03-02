use std::sync::{Mutex, RwLock};
use std::thread;
use std::time::{Instant, Duration};
use std::{sync::Arc, collections::HashMap};
use std::io::Error;
use crossbeam::channel::{unbounded, Sender, Receiver};
use uuid::Uuid;
use super::models::bar::Bar;
use super::models::index::DatabaseIndex;
use super::models::query::Query;
use super::tasks::Task;
use super::tasks::query::QueryTask;
use super::{models::{barset::BarSet, query_result::QueryResult}, cache::InMemoryCache, threads::ThreadPool};


// use a dynamically sized thread pool to query the database filesystem
#[derive(Clone)]
pub struct DatabaseEngine {
    // the root path to the files that contain the data
    pub path: String,

    // index of the files in the database
    // pub index: 

    // the thread pool that performs the queries and the sub-tasks within each query
    thread_pool: Arc<ThreadPool>,

    // the cache that stores a queue of synchronized and consolidated data to be passed to the client
    cache: Arc<Mutex<InMemoryCache>>,

    // the hashmap that stores the query channels
    query_channels: HashMap<String, Arc<Receiver<BarSet>>>,
}

impl DatabaseEngine {
    // creates a new database engine
    pub fn new() -> DatabaseEngine {
        // create a thread pool for the database engine
        let thread_pool = Arc::new(ThreadPool::new(4));

        // the cache that stores a queue of synchronized and consolidated data to be passed to the client
        let cache = Arc::new(Mutex::new(InMemoryCache::new()));

        // create an index of the files in the database

        DatabaseEngine {
            path: "./data".to_string(),
            thread_pool,
            cache,
            query_channels: HashMap::new(),
        }
    }

    // create a new database engine with a given path
    pub fn new_with(path: String, min_workers: i8) -> DatabaseEngine {
        // create a thread pool for the database engine
        let thread_pool = Arc::new(ThreadPool::new(min_workers as usize));

        // the cache that stores a queue of data to be passed to the client
        let cache = Arc::new(Mutex::new(InMemoryCache::new()));
        
        DatabaseEngine {
            path,
            thread_pool,
            cache,
            query_channels: HashMap::new(),
        }
    }

    // query the database for bars
    pub fn start_query(&mut self, client_id: u64, query: Query) -> Result<QueryResult, Error> {
        // generate a uuid for the query id
        let query_id = format!("{}_{}", client_id, Uuid::new_v4().to_string());
        let query_id_thread = query_id.clone();
        let query_id_task = query_id.clone();

        // use index to look up the files that contain the data for the query
        let index = DatabaseIndex::from_query("index.json".to_string(), "data".to_string(), query.clone());
        let index_clone = index.clone();
        let path = self.path.clone();
        let filenames = index_clone.corpus_map.clone().values().map(|s| {
            format!("{}/{}.stmdb", path, s.filename.to_string())
        }).collect::<Vec<String>>();

        // create a new query channel and store it in a hashmap
        let (query_channel, receiver_channel): (Sender<BarSet>, Receiver<BarSet>) = unbounded();
        let query_channel = Arc::new(query_channel);
        let receiver_channel = Arc::new(receiver_channel);
        self.query_channels.insert(query_id.clone(), receiver_channel.clone());

        // start the tasks that need to be started on the thread pool to perform the query
        // - multiple tasks to read a chunk from a file
        // - a single task to synchronize the the results from read tasks
        // - a single task to consolidate different intervals of the data
        let thread_pool = self.thread_pool.clone();
        let cache = self.cache.clone();
        let mut query_channels = self.query_channels.clone();
        self.thread_pool.execute(move || {
            println!("thread.query: starting query");
            let start = Instant::now();

            // create a new query task
            let limit = query.limit;
            let mut task = QueryTask::new(
                thread_pool,
                query_channel.clone(),
                filenames.clone(),
                query.clone().start_timestamp.unwrap(),
                query.clone().end_timestamp.unwrap(),
                limit
            );

            // start the query task
            task.execute(Some(Box::new(move |result: bool| {
                let elapsed = start.elapsed().as_millis();
                // print the result of the query task
                println!("query {} completed in {}ms", query_id_task.clone(), elapsed);
            })));

            // wait for results on the channel in a loop until there are none left
            println!("thread.query: waiting for results");
            while let Ok(results) = receiver_channel.recv() {

                // println!("thread.query: got results: {:?}", results.bars.get(100).unwrap());

                // try to get the lock on the cache
                let mut cache = cache.lock().unwrap();

                // add the results to the cache
                cache.add(query_id_thread.clone(), results.bars.clone(), results.is_last.clone());

                // sleep on all by the last result
                if !results.is_last {
                    thread::sleep(Duration::from_micros(10));
                }
            }

            // remove the query channel from the hashmap when it is done
            query_channels.remove(&query_id_thread.clone());
        });

        Ok(QueryResult::new(query_id, "running".to_string()))
    }

    // query the database for bars with a given query id from a first query
    pub fn query_chunk(&mut self, query_id: String, parameters: HashMap<String, String>) -> Result<QueryResult, Error> {
        
        // get limit parameter from the parameters hashmap
        let limit = parameters.get("limit").unwrap_or(&"1000".to_string()).parse::<i32>().unwrap();

        // get the lock for the cache
        let mut cache = self.cache.lock().unwrap();

        // check if the query id exists in the cache
        let cache_result = cache.get(query_id.clone(), limit);
        if let Some(cache_result) = cache_result {
            if cache_result.bars.len() > 0 {
                let status = if cache_result.is_last { "complete" } else { "running" };
            
                // return the cached results
                return Ok(QueryResult::new_with(query_id, status.to_string(), cache_result.bars));
            }
        }

        // check if the query id exists in the query channels hashmap
        if !self.query_channels.contains_key(&query_id) {
            return Err(Error::new(std::io::ErrorKind::Other, "engine: query id does not exist"));
        }

        // wait for results on the channel in a loop until there are none left
        let query_channel = self.query_channels.get(&query_id).unwrap().clone();
        
        // wait for results on the channel in a loop until there are none left
        // println!("engine: waiting for query results");
        match query_channel.recv_timeout(Duration::from_micros(10)) {
            Ok(results) => {
                // last page of results
                if results.is_last {
                    return Ok(QueryResult::new_with(query_id, "complete".to_string(), results.bars));
                }

                // println!("engine: got query results");
                return Ok(QueryResult::new_with(query_id, "running".to_string(), results.bars));
            },
            _ => (),
            // Err(e) => {
            //     println!("{:?}", e);
            //     println!("engine: no results");
            //     return Err(Error::new(std::io::ErrorKind::Other, "engine: no results"));
            // }
        }

        Ok(QueryResult::new_with(query_id, "running".to_string(), vec![]))
    }
}