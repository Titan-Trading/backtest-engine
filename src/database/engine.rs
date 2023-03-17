use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use std::time::{Instant, Duration};
use std::{sync::Arc, collections::HashMap};
use std::io::{Error, SeekFrom, Seek, ErrorKind};
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender, unbounded};
use uuid::Uuid;
use super::models::candlestick::Candlestick;
use super::models::header::Header;
use super::models::index::{DatabaseIndex, Corpus};
use super::models::query::Query;
use super::tasks::Task;
use super::tasks::perform_query::QueryTask;
use super::{models::{barset::BarSet, query_result::QueryResult}, cache::InMemoryCache, threads::ThreadPool};


// use a dynamically sized thread pool to query the database filesystem
#[derive(Clone)]
pub struct DatabaseEngine {
    // the root path to the files that contain the data
    pub path: String,

    // index of the files in the database
    pub index: DatabaseIndex,

    // the thread pool that performs the queries and the sub-tasks within each query
    thread_pool: Arc<ThreadPool>,

    // the cache that stores a queue of synchronized and consolidated data to be passed to the client
    cache: Arc<Mutex<InMemoryCache>>,

    // the hashmap that stores the task receiver channels
    task_channels: HashMap<String, Arc<Receiver<BarSet>>>,
}

impl DatabaseEngine {
    // creates a new database engine
    pub fn new() -> DatabaseEngine {

        // file names/paths
        let root_data_dir = "./data".to_string();
        let index_filename = format!("{}/{}", &root_data_dir, "index.json");

        // create a thread pool for the database engine
        // let thread_pool = Arc::new(Builder::new_multi_thread().worker_threads(4).build().unwrap());
        let thread_pool = Arc::new(ThreadPool::new(4));

        // the cache that stores a queue of synchronized and consolidated data to be passed to the client
        let cache = Arc::new(Mutex::new(InMemoryCache::new()));

        // create an index of the files in the database
        let index =DatabaseIndex::new(index_filename, root_data_dir.clone());

        DatabaseEngine {
            path: root_data_dir,
            index: index,
            thread_pool,
            cache,
            task_channels: HashMap::new(),
        }
    }

    // create a new database engine with a given path
    pub fn new_with(path: String, min_workers: i8) -> DatabaseEngine {

        // file names/paths
        let index_filename = "index.json".to_string();

        // create a thread pool for the database engine
        // let thread_pool = Arc::new(Builder::new_multi_thread().worker_threads(min_workers as usize).build().unwrap());
        let thread_pool = Arc::new(ThreadPool::new(min_workers as usize));

        // the cache that stores a queue of data to be passed to the client
        let cache = Arc::new(Mutex::new(InMemoryCache::new()));

        // read the index file
        let index = DatabaseIndex::new(index_filename, path.clone());
        
        DatabaseEngine {
            path,
            index,
            thread_pool,
            cache,
            task_channels: HashMap::new(),
        }
    }

    // get the database index
    pub fn get_index(&self) -> DatabaseIndex {
        self.index.clone()
    }

    // query the database for bars
    pub fn start_query(&mut self, client_id: u64, query: Query) -> Result<QueryResult, Error> {
        
        // generate a uuid for the query id
        let query_id = format!("{}_{}", client_id, Uuid::new_v4().to_string());

        // use index to look up the files that contain the data for the query
        let index_query = self.index.from_query(query.clone());
        let filenames = index_query.corpus_map.clone().values().map(|s| {
            format!("{}/{}.stmdb", self.path, s.filename.to_string())
        }).collect::<Vec<String>>();

        println!("{:?}", filenames);

        // create a new task output and main thread receiver channels
        let (task_output_channel, receiver_channel): (Sender<BarSet>, Receiver<BarSet>) = unbounded();
        
        // create a new output for query tasks channel mutex
        let task_output_channel = Arc::new(Mutex::new(task_output_channel));
        
        // create a new main thread receiver channel mutex
        let receiver_channel = Arc::new(receiver_channel);
        self.task_channels.insert(query_id.clone(), receiver_channel.clone());

        // start the tasks that need to be started on the thread pool to perform the query
        // - multiple tasks to read a chunk from a file
        // - a single task to synchronize the the results from read tasks
        // - a single task to consolidate different intervals of the data
        let query_pool = self.thread_pool.clone();
        let cache = self.cache.clone();

        // spawn a new task on the thread pool
        let query_id_task = query_id.clone();
        let query_id_task_clone = query_id.clone();
        let query_channel_task = task_output_channel.clone();
        let query_filenames = filenames.clone();
        let query_cache = cache.clone();
        self.thread_pool.execute(move || {
            println!("thread.query: starting query");
            let start = Instant::now();

            // create a new query task
            let limit = query.limit;
            let mut task = QueryTask::new(
                query_pool.clone(),
                query_channel_task,
                query_filenames,
                query.clone().start_timestamp.unwrap(),
                query.clone().end_timestamp.unwrap(),
                limit
            );

            // start the query task
            task.execute(Some(Box::new(move |result: bool| {
                let elapsed = start.elapsed().as_millis();
                // print the result of the query task
                println!("query {} completed in {}ms", query_id_task, elapsed);
            })));

            // wait for results on the channel in a loop until there are none left
            println!("thread.query: waiting for results");
            while let Ok(results) = receiver_channel.try_recv() {
                let barset = results;

                // println!("thread.query: got results: {:?}", results.bars.get(100).unwrap());

                // try to get the lock on the cache
                let mut cache = query_cache.lock().unwrap();

                // add the results to the cache
                cache.add(query_id_task_clone.clone(), barset.bars, barset.is_last);

                // sleep on all by the last result
                if !barset.is_last {
                    thread::sleep(Duration::from_micros(10));
                }
            }
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
        if !self.task_channels.contains_key(&query_id) {
            return Err(Error::new(std::io::ErrorKind::Other, "engine: query id does not exist"));
        }

        // wait for results on the channel in a loop until there are none left
        let query_channel = self.task_channels.get(&query_id).unwrap().clone();
        
        // wait for results on the channel in a loop until there are none left
        match query_channel.try_recv() {
            Ok(results) => {
                // last page of results
                if results.is_last {
                    return Ok(QueryResult::new_with(query_id, "complete".to_string(), results.bars));
                }

                // println!("engine: got query results");
                return Ok(QueryResult::new_with(query_id, "running".to_string(), results.bars));
            },
            _ => (),
        }

        Ok(QueryResult::new_with(query_id, "running".to_string(), vec![]))
    }

    // insert data into the database
    pub fn insert(&mut self, client_id: u64, exchange: String, symbol: String, data: Vec<Candlestick>) -> Result<bool, Error> {
        
        // generate a uuid for the query id
        let query_id = format!("{}_{}", client_id, Uuid::new_v4().to_string());

        // get minimum and maximum timestamps from the data
        let chunk_start_timestamp = data.iter().map(|x| x.timestamp).min().unwrap();
        let chunk_end_timestamp   = data.iter().map(|x| x.timestamp).max().unwrap();

        // look up the exchange and symbol in the index
        let index_result = self.index.get_info((exchange.clone(), symbol.clone()));
        let corpus = match index_result {
            Ok(corpus) => {
                corpus
            },
            Err(_) => {
                // create a new dataset id
                let file_id = self.index.last_file_id + 1;

                // get current unix utc timestamp
                let now = Utc::now().timestamp();

                // get file name for the dataset
                let filename = format!("{}_{}.stmdb", exchange, symbol);

                // create a new corpus for the exchange and symbol
                let new_corpus = Corpus::new(file_id, exchange.clone(), symbol.clone(), chunk_start_timestamp, chunk_end_timestamp, filename, now);
                
                // add the corpus to the index
                self.index.add_file(new_corpus.clone()).unwrap();
                self.index.last_file_id = file_id;

                new_corpus
            }
        };

        let mut file_handle: File;
        let mut header: Header;
        let mut file_start_timestamp: i64;
        let mut file_end_timestamp: i64;

        // check if the file exists
        let filename = format!("{}/{}_{}.stmdb", self.path, exchange, symbol);
        if !Path::new(&filename).exists() {
            // create a new file if the file doesn't exist
            file_handle = File::create(&filename).unwrap();

            // create a new file header
            header = Header::new(corpus.file_id as u32, chunk_start_timestamp as u64, chunk_end_timestamp as u64);
            header.into_writer(&mut file_handle).unwrap();

            // set the start and end timestamps
            file_start_timestamp = chunk_start_timestamp;
            file_end_timestamp   = chunk_end_timestamp;
        }
        else {
            // open the file
            file_handle = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&filename)
                .unwrap();

            // read the file header
            header = Header::from_file(&mut file_handle).unwrap();

            // set the start and end timestamps
            file_start_timestamp = header.start_timestamp as i64;
            file_end_timestamp   = header.end_timestamp as i64;

            // check if the chunk is outside the range of the file
            if chunk_start_timestamp < file_start_timestamp {
                file_start_timestamp = chunk_start_timestamp;
            }
            if chunk_end_timestamp > file_end_timestamp {
                file_end_timestamp = chunk_end_timestamp;
            }

            // update file header
            header.update(&mut file_handle, corpus.file_id as u32, file_start_timestamp as u64, file_end_timestamp as u64).unwrap();
        }

        // seek to the end of the file
        file_handle.seek(SeekFrom::End(0)).unwrap();

        // write the data to the file
        for mut candlestick in data {

            // skip if the candlestick is outside the range of the file
            if candlestick.timestamp < corpus.start_timestamp || candlestick.timestamp > corpus.end_timestamp {
                // continue;
            }

            // write/append a candlestick to the file
            candlestick.into_writer(&mut file_handle).unwrap();
        }

        // close the file
        file_handle.sync_all().unwrap();
        
        // update the index (write to disk)
        self.index.save().unwrap();
        
        Ok(true)
    }
}