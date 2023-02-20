
// basic thread management
// - stop, start, pause and resume threads
// - handle errors from within threads
// - join main thread with child threads so that all the children stop before the main thread
// - send or receive messages to or from a thread

use std::{thread, sync::{Mutex, Arc}, time::Duration, collections::HashMap};

#[derive(Clone, Copy)]
pub enum ThreadState {
    Running,
    Paused,
    Stopped
}

#[derive(Clone)]
pub struct ThreadManager {
    pub threads: HashMap<String, Arc<Mutex<ThreadState>>>,
}

impl ThreadManager {
    pub fn new() -> ThreadManager {
        ThreadManager {
            threads: HashMap::new()
        }
    }

    // state a named thread using a closure
    pub fn start<F>(&mut self, name: String, f: F) -> bool
    where
        F: Fn() + Send + 'static,
    {
        let state = Arc::new(Mutex::new(ThreadState::Running));
        let state_clone = state.clone();

        thread::spawn(move || {
            loop {
                match *state_clone.lock().unwrap() {
                    ThreadState::Running => f(),
                    ThreadState::Paused => thread::sleep(Duration::from_millis(10)),
                    ThreadState::Stopped => break,
                }
            }
        });

        self.threads.insert(name.clone(), state);

        true
    }

    // stop a thread by name
    pub fn stop(&mut self, name: String) -> bool {
        if let Some(state) = self.threads.get(name.as_str()) {
            *state.lock().unwrap() = ThreadState::Stopped;

            return true;
        }

        false
    }

    // pause a thread by name
    pub fn pause(&mut self, name: String) -> bool {
        if let Some(state) = self.threads.get(name.as_str()) {
            *state.lock().unwrap() = ThreadState::Paused;

            return true;
        }

        false
    }

    // resume a thread by name
    pub fn resume(&mut self, name: String) -> bool {
        if let Some(state) = self.threads.get(name.as_str()) {
            *state.lock().unwrap() = ThreadState::Running;

            return true;
        }

        false
    }

    // get the state of a thread by name
    pub fn get_state(&mut self, name: String) -> String {
        if let Some(state) = self.threads.get(name.as_str()) {
            // let state_clone = state.lock().unwrap();
            let current_state = *state.lock().unwrap();
            match current_state {
                ThreadState::Running => return "running".to_string(),
                ThreadState::Paused => return "paused".to_string(),
                ThreadState::Stopped => return "stopped".to_string()
            }
        }

        "stopped".to_string()
    }
}