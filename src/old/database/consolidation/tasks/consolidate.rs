use std::sync::{Mutex, Arc};

use crossbeam::channel::Sender;

use crate::database::{threads::Task, database::{BarSet, ConsolidatedBarSet}};


// a job that performs a one or multiple reads from a file
pub struct ConsolidateTask {
    channel: Arc<Mutex<Sender<Vec<ConsolidatedBarSet>>>>,
    intervals: Vec<String>,
    barsets: Vec<BarSet>,
}
impl ConsolidateTask {

    // create a new read file task
    pub fn new(channel: Arc<Mutex<Sender<Vec<ConsolidatedBarSet>>>>, intervals: Vec<String>, barsets: Vec<BarSet>) -> Self {
        Self {
            channel,
            intervals,
            barsets,
        }
    }
}
impl Task for ConsolidateTask {

    // execute the read file task
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>) {
        
        println!("thread.tasks.consolidate: consolidating barsets with {} intervals", self.intervals.len());

        let mut results: Vec<ConsolidatedBarSet> = Vec::new();

        // consolidate the barset using the given intervals
        for interval in self.intervals.clone() {
            println!("thread.tasks.consolidate: consolidating barsets with {}", interval);
            let consolidated_barsets = consolidate_data(&mut self.barsets.clone(), interval.clone());
        
            results.extend(consolidated_barsets);
        }

        // try to get the lock
        let mut channel = match self.channel.lock() {
            Ok(channel) => channel,
            Err(e) => {
                println!("thread.tasks.consolidate: error getting lock on channel: {}", e);
                return;
            }
        };

        // send the consolidated barset back to the main thread
        match channel.send(results) {
            Ok(_) => {
                println!("thread.tasks.consolidate: consolidated barsets sent to main thread");
            },
            Err(e) => {
                println!("thread.tasks.consolidate: error sending consolidated barsets to main thread: {}", e);
            }
        }
    }
}

// consolidate a group of barsets into a single interval
fn consolidate_data(barsets: &mut Vec<BarSet>, interval: String) -> Vec<ConsolidatedBarSet> {
    let mut consolidated_barsets: Vec<ConsolidatedBarSet> = Vec::new();

    for barset in barsets {
        let consolidated_barset = consolidate_barset(barset, interval.clone());
        consolidated_barsets.push(consolidated_barset);
    }

    consolidated_barsets
}

// consolidate a barset into a single interval
fn consolidate_barset(barset: &mut BarSet, interval: String) -> ConsolidatedBarSet {
    let mut consolidated_barset = ConsolidatedBarSet::new();

    consolidated_barset
}