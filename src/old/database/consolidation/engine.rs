use std::{io::{Error, ErrorKind}, sync::{Mutex, Arc}};
use crossbeam::channel::{bounded, Receiver, Sender, unbounded};

use crate::database::{threads::{ThreadPool, Task}, database::{Bar, BarSet, ConsolidatedBarSet}};

use super::tasks::consolidate::ConsolidateTask;


// create a thread pool that exclusively handles consolidation
// consolidation is a very expensive operation
// we could potentially be taking records from hundreds of files
pub struct ConsolidationEngine {
    pub thread_pool: Arc<ThreadPool>,
}

impl ConsolidationEngine {

    // create a new consolidation engine
    pub fn new(thread_pool_size: i8) -> ConsolidationEngine {
        let thread_pool = Arc::new(ThreadPool::new(thread_pool_size as usize));

        ConsolidationEngine {
            thread_pool,
        }
    }

    // create a new consolidation engine with a given thread pool
    pub fn new_with(thread_pool: Arc<ThreadPool>) -> ConsolidationEngine {
        ConsolidationEngine {
            thread_pool,
        }
    }

    // check if the interval is supported
    pub fn is_interval_supported(&self, interval: String) -> bool {
        let supported_intervals = vec!["1m", "5m", "15m", "1h", "1d"];
        match interval.chars().last().unwrap() {
            'm' | 'h' | 'd' | 'w' | 'M' | 'y' => {
                // do nothing
            },
            _ => {
                return false;
            },
        }

        true
    }

    // consolidate records into multiple intervals
    pub fn consolidate(&mut self, intervals: Vec<String>, barsets: Vec<BarSet>) -> Result<Vec<ConsolidatedBarSet>, Error> {
        
        // check if the interval is supported
        for interval in intervals.clone() {
            if !self.is_interval_supported(interval) {
                return Err(Error::new(ErrorKind::Other, "Interval not supported"));
            }
        }

        // Create a channel for sending results from the worker threads
        let (tx, rx): (Sender<Vec<ConsolidatedBarSet>>, Receiver<Vec<ConsolidatedBarSet>>) = unbounded();

        // wrap the channel in an Arc to share it between threads
        let tx = Arc::new(Mutex::new(tx));

        // if more than 1000 barsets, then we need to split the consolidation into multiple tasks
        // each task will consolidate 1000 barsets
        // we will then consolidate the results of each task into a vector of consolidated barsets
        let barsets_copy = barsets.clone();
        for i in (0..barsets_copy.len()).step_by(1000) {
            println!("Creating consolidation task for barsets {} to {}", i, i+1000);
            
            // create a chunk of barsets
            let barsets_chunk = barsets_copy[i..i+1000].to_vec();

            // create consolidation task
            let mut task = ConsolidateTask::new(Arc::clone(&tx), intervals.clone(), barsets_chunk);
                    
            // pass task to thread pool to be executed
            self.thread_pool.execute(move || {
                task.execute(Some(Box::new(|result: bool| {
                    println!("task.execute result: {:?}", result);
                })));
            });
        }

        // create consolidation task
        let mut task = ConsolidateTask::new(Arc::clone(&tx), intervals, barsets);
                
        // pass task to thread pool to be executed
        self.thread_pool.execute(move || {
            task.execute(None);
        });

        // create a vector of consolidated barsets for results
        let mut results: Vec<ConsolidatedBarSet> = Vec::new();

        // loop through all barsets waiting for the thread pool tasks to finish
        for _ in 0..barsets_copy.len() {
            println!("Waiting for consolidation results");

            // wait for a result
            let consolidated_barsets = rx.recv().unwrap();

            // add the consolidated barsets to the results
            results.extend(consolidated_barsets);
        }

        Ok(results)
    }
}