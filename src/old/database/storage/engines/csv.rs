use std::collections::HashMap;
use std::rc::Rc;
use std::{thread, sync::Arc};
use std::io::Error;
use crate::database::database::{BarSet, Bar};
use crate::database::storage::corpus::{CorpusEngine, CorpusIndex};
use crate::database::threads::ThreadPool;
use crate::filesystem::storage::FileSystem;


// use filesystem to read files into records
// will use multiple threads to read files
// use filesystem to write files from records

pub struct CSVCorpusEngine {
    pub filesystem: FileSystem,
    pub thread_pool: Arc<ThreadPool>,
    pub corpus_index: Arc<CorpusIndex>,
}

impl CSVCorpusEngine {
    
    // create a new corpus engine
    pub fn new(thread_pool: Arc<ThreadPool>, corpus_index: Arc<CorpusIndex>) -> CSVCorpusEngine {
        // connect to our filesystem
        let filesystem = FileSystem::connect();

        CSVCorpusEngine {
            thread_pool,
            filesystem,
            corpus_index,
        }
    }

    // create a new corpus engine with a filesystem
    pub fn new_with(filesystem: FileSystem, thread_pool: Arc<ThreadPool>, corpus_index: Arc<CorpusIndex>) -> CSVCorpusEngine {
        CSVCorpusEngine {
            thread_pool,
            filesystem,
            corpus_index,
        }
    }
}

impl CorpusEngine for CSVCorpusEngine {

    // read list of files from the filesystem using multiple threads (whole files)
    fn read_files(&mut self, files: Vec<String>, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error> {

        // here's the process
        // 1. get list of files to process from
        // 2. start a thread for each file or use a thread pool
        // 3. once a record is read from all files, synchronize the data into a single bar structure
        // 4. return that bar of data
        // 5. have the threads continue reading files and writing the data into a cache structure
        // 6. when available data is requested check the cache first

        let mut thread = thread::spawn(move || {
            println!("Hello, world!");
        });

        let result: HashMap<String, Vec<Bar>> = HashMap::new();

        Ok(result)
    }

    // read list of files from the filesystem using multiple threads (chunks)
    fn read_chunk(&mut self, files: Vec<String>, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error> {
        let result: HashMap<String, Vec<Bar>> = HashMap::new();
        Ok(result)
    }

    // write list of files to the filesystem using multiple threads
    fn write_files(&mut self, files: Vec<String>) -> Result<Vec<bool>, Error> {
        let result: Vec<bool> = Vec::new();
        Ok(result)
    }
}