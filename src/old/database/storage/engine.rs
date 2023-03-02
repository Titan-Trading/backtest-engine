use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::io::Error;
use crate::database::database::{Bar, BarSet};
use crate::database::query::QueryResult;
use crate::database::threads::ThreadPool;
use crate::database::{engine::DatabaseEngineType, query::Query};
use crate::filesystem::storage::FileSystem;
use crate::filesystem::reader::{Reader, ReaderResult};
use crate::filesystem::types::FileType;
use super::corpus::{CorpusIndex, Corpus, CorpusEngine};
use super::engines::csv::CSVCorpusEngine;
use super::engines::stmdb::STMDBCorpusEngine;


// represents a storage engine
// manages a thread pool for reading and writing data
pub struct StorageEngine {
    pub engine_type: DatabaseEngineType,
    pub filesystem: FileSystem,
    pub file_thread_pool: Arc<ThreadPool>,
    pub corpus_engine: Box<dyn CorpusEngine + Send>,
    pub corpus_index: Arc<CorpusIndex>,
}

impl StorageEngine {

    // create a new storage engine given a database engine type
    pub fn new(engine_type: DatabaseEngineType) -> StorageEngine {
        // create a new filesystem connection
        let filesystem = FileSystem::connect();

        // create a new file thread pool
        let file_thread_pool = Arc::new(ThreadPool::new(1));

        // create a new corpus index (read from index.json)
        // a corpus is a collection of files
        // a corpus index is a collection of files names and their metadata
        let corpus_index: Arc<CorpusIndex>;
        let parameters = HashMap::new();
        let reader_result = Reader::new_with(FileType::Json).read_file("index.json".to_string(), parameters).unwrap();
        if let ReaderResult::STMDBIndex(corpus_index_content) = reader_result {
            // convert file system map to a hashmap
            let mut corpus_map = HashMap::new();
            for (key, value) in corpus_index_content.file_map {
                let dataset_id = value.dataset_id;
                let last_updated = value.last_updated;
                let exchange = value.exchange;
                let symbol = value.symbol;
                let start_timestamp = value.start_timestamp;
                let end_timestamp = value.end_timestamp;
                // let filename = value.filename;

                let filename = format!("{}.stmdb", key);
                let corpus = Corpus::new(dataset_id as u8, exchange, symbol, start_timestamp, end_timestamp, filename, last_updated);
                corpus_map.insert(key, corpus);
            }

            corpus_index = Arc::new(CorpusIndex::new_from("index.json".to_string(), filesystem.cwd.clone(), corpus_map));
        }
        else {
            panic!("Corpus index is not valid STMDBIndex");
        }

        // create a new corpus engine using thread pool
        let corpus_engine: Box<dyn CorpusEngine + Send> = match engine_type {
            DatabaseEngineType::Stmdb => Box::new(STMDBCorpusEngine::new(file_thread_pool.clone(), corpus_index.clone())),
            DatabaseEngineType::Csv => Box::new(CSVCorpusEngine::new(file_thread_pool.clone(), corpus_index.clone())),
            _ => panic!("Unsupported storage engine"),
        };

        StorageEngine {
            engine_type,
            filesystem,
            file_thread_pool,
            corpus_engine,
            corpus_index,
        }
    }

    // create a new storage engine given a database engine type and an existing thread pool
    pub fn new_with(engine_type: DatabaseEngineType, file_thread_pool: Arc<ThreadPool>) -> StorageEngine {
        // create a new filesystem connection
        let filesystem = FileSystem::connect();
        
        // create a new corpus index (read from index.json)
        // a corpus is a collection of files
        // a corpus index is a collection of files names and their metadata
        let corpus_index: Arc<CorpusIndex>;
        let parameters = HashMap::new();
        let reader_result = Reader::new_with(FileType::Json).read_file("index.json".to_string(), parameters).unwrap();
        if let ReaderResult::STMDBIndex(corpus_index_content) = reader_result {
            // convert file system map to a hashmap
            let mut corpus_map = HashMap::new();
            for (key, value) in corpus_index_content.file_map {
                let dataset_id = value.dataset_id;
                let last_updated = value.last_updated;
                let exchange = value.exchange;
                let symbol = value.symbol;
                let start_timestamp = value.start_timestamp;
                let end_timestamp = value.end_timestamp;
                // let filename = value.filename;

                let filename = format!("{}.stmdb", key);
                let corpus = Corpus::new(dataset_id as u8, exchange, symbol, start_timestamp, end_timestamp, filename, last_updated);
                corpus_map.insert(key, corpus);
            }

            corpus_index = Arc::new(CorpusIndex::new_from("index.json".to_string(), filesystem.cwd.clone(), corpus_map));
        }
        else {
            panic!("Corpus index is not valid STMDBIndex");
        }

        // create a new corpus engine using thread pool
        let corpus_engine: Box<dyn CorpusEngine + Send> = match engine_type {
            DatabaseEngineType::Stmdb => Box::new(STMDBCorpusEngine::new(file_thread_pool.clone(), corpus_index.clone())),
            DatabaseEngineType::Csv => Box::new(CSVCorpusEngine::new(file_thread_pool.clone(), corpus_index.clone())),
            _ => panic!("Unsupported storage engine"),
        };

        // create a new 
        StorageEngine {
            engine_type,
            filesystem,
            file_thread_pool,
            corpus_engine,
            corpus_index,
        }
    }

    // query the storage engine using corpus engine for filetype and parameters
    pub fn query(&mut self, query: Query, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error> {

        // convert query to list of files that will need to be read
        let mut query_files_index = CorpusIndex::from_query("index.json".to_string(), self.filesystem.cwd.clone(), query);

        // convert query to list of files that will need to be read
        let mut files = Vec::new();
        let corpus_map = self.corpus_index.corpus_map.clone();
        for (key, corpus) in corpus_map {

            // check if the file is in the query files
            if !query_files_index.corpus_map.contains_key(&key) {
                continue;
            }

            // add the filename to the list of files to read
            files.push(corpus.filename)
        }
        println!("engine [storage]: reading {} files", files.len());

        // use the corpus engine to read the files from the filesystem
        // start a thread for each file that needs to be read
        let results = self.corpus_engine.read_chunk(files, parameters)?;
        println!("engine [storage]: corpus engine results: {:?}", results);

        // create some object to store file indices

        Ok(results)
    }
}
