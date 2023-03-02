use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Mutex;
use std::{sync::Arc};
use std::io::Error;
use crossbeam::channel::{unbounded, Sender, Receiver};

use crate::database::database::BarSet;
use crate::database::storage::corpus::{CorpusEngine, CorpusIndex};
use crate::database::storage::synchronizer::Synchronizer;
use crate::database::storage::tasks::file_chunks::ReadChunkTask;
use crate::database::threads::Task;
use crate::database::storage::tasks::files::{ReadTask};
use crate::database::threads::ThreadPool;
use crate::filesystem::storage::FileSystem;
use crate::filesystem::reader::Reader;
use crate::filesystem::types::stmdb::STMDBRecord;
use crate::{database::{database::Bar}};

// use filesystem to read files into records
// will use multiple threads to read files
// use filesystem to write files from records

pub struct STMDBCorpusEngine {
    pub filesystem: FileSystem,
    pub thread_pool: Arc<ThreadPool>,
    pub corpus_index: Arc<CorpusIndex>,
}

impl STMDBCorpusEngine {

    // create a new corpus engine
    pub fn new(thread_pool: Arc<ThreadPool>, corpus_index: Arc<CorpusIndex>) -> STMDBCorpusEngine {
        // connect to our filesystem
        let mut filesystem = FileSystem::connect();
        filesystem.set_cwd("./data".to_string());

        STMDBCorpusEngine {
            thread_pool,
            filesystem,
            corpus_index,
        }
    }

    // create a new corpus engine with a filesystem
    pub fn new_with(filesystem: FileSystem, thread_pool: Arc<ThreadPool>, corpus_index: Arc<CorpusIndex>) -> STMDBCorpusEngine {
        STMDBCorpusEngine {
            thread_pool,
            filesystem,
            corpus_index,
        }
    }
}

impl CorpusEngine for STMDBCorpusEngine {

    // read list of files from the filesystem using multiple threads (whole files)
    fn read_files(&mut self, files: Vec<String>, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error> {
        
        // create a channel
        let (sender, receiver): (Sender<Vec<STMDBRecord>>, Receiver<Vec<STMDBRecord>>) = unbounded();
        let sender = Arc::new(Mutex::new(sender));

        // create a task for each file
        for file in files.clone() {
            println!("corpus engine [stmdb]: reading file {}", file);

            let reader = Reader::new();

            // create task
            let mut task = ReadTask::new(reader, sender.clone(), file.clone(), parameters);

            // pass task to thread pool to be executed
            self.thread_pool.execute(move || {
                println!("pool.execute: start read task on file {}", file);
                task.execute(Some(Box::new(|result: bool| {
                    println!("thread.tasks.files result: {:?}", result);
                })));
            });
        }

        // wait for all tasks to complete
        // get the chunks from all files
        let mut bar_chunks: HashMap<String, Vec<STMDBRecord>> = HashMap::new();
        for file in files.clone() {
            println!("corpus engine [stmdb]: waiting for file {}", file);
            let result = receiver.recv();
            
            // wait for a bar to be received
            if let Err(e) = result {
                println!("corpus engine [stmdb]: unable to receive records from file read task: {:?}", e);
                continue;
            }

            let records = result.unwrap();
            println!("corpus engine [stmdb]: received {} records from file {}", records.len(), file);

            bar_chunks.insert(file, records);
        }
        println!("corpus engine [stmdb]: all files read");
        println!("corpus engine [stmdb]: {} bar chunks", bar_chunks.len());

        // synchronize the data into bar sets across all files
        println!("corpus engine [stmdb]: synchronizing data into bar sets");
        let mut synchronizer = Synchronizer::new(files.clone(), bar_chunks.clone());

        // loop through all records from the first file
        let mut barsets: Vec<BarSet> = Vec::new();
        for record in bar_chunks.get(&files[0]).unwrap() {
            let timestamp = record.timestamp;
            
            // call synchronizer.sync() to get the matching records from all files
            let barset = synchronizer.sync(timestamp);
            barsets.push(barset.clone());

            println!("corpus engine [stmdb]: barset: {:?}", barset);
        }
        
        println!("corpus engine [stmdb]: all bar chunks synchronized");
        println!("corpus engine [stmdb]: {} bar sets", barsets.len());

        // here's the process
        // 1. get list of files to process from
        // 2. start a thread for each file or use a thread pool
        // 3. once a record is read from all files, synchronize the data into a single bar structure
        // 4. return that bar of data
        // 5. have the threads continue reading files and writing the data into a cache structure
        // 6. when available data is requested check the cache first

        let result: HashMap<String, Vec<Bar>> = HashMap::new();

        Ok(result)
    }

    // read list of files from the filesystem using multiple threads (chunks)
    fn read_chunk(&mut self, files: Vec<String>, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error> {
        
        // create a channel
        let (sender, receiver): (Sender<Vec<STMDBRecord>>, Receiver<Vec<STMDBRecord>>) = unbounded();
        let sender = Arc::new(Mutex::new(sender));

        // create a task for each file chunk
        for file in files.clone() {
            println!("corpus engine [stmdb]: reading file chunk {} {:?}", file, parameters.clone());

            let reader = Reader::new();

            // create task
            let mut task = ReadChunkTask::new(reader, sender.clone(), file.clone(), parameters);

            // pass task to thread pool to be executed
            self.thread_pool.execute(move || {
                println!("pool.execute: start read chunk task on file {}", file);
                task.execute(Some(Box::new(|result: bool| {
                    println!("thread.tasks.file_chunks result: {:?}", result);
                })));
            });
        }

        // wait for all tasks to complete
        // get the chunks from all files
        let mut bar_chunks: HashMap<String, Vec<STMDBRecord>> = HashMap::new();
        for file in files.clone() {
            println!("corpus engine [stmdb]: waiting for file chunk {}", file);
            let result = receiver.recv();
            
            // wait for a bar to be received
            if let Err(e) = result {
                println!("corpus engine [stmdb]: unable to receive records from file read chunk task: {:?}", e);
                continue;
            }

            let records = result.unwrap();
            println!("corpus engine [stmdb]: received {} records from file {}", records.len(), file);

            bar_chunks.insert(file, records);
        }
        println!("corpus engine [stmdb]: all file chunks read");
        println!("corpus engine [stmdb]: {} bar chunks", bar_chunks.len());

        let result: HashMap<String, Vec<Bar>> = HashMap::new();
        Ok(result)
    }

    // write list of files to the filesystem using multiple threads
    fn write_files(&mut self, files: Vec<String>) -> Result<Vec<bool>, Error> {
        let result: Vec<bool> = Vec::new();
        Ok(result)
    }
}