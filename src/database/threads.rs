use std::{sync::{Arc, Mutex}, thread};
use crossbeam::channel::{Sender, unbounded, Receiver};


// setup thread pooling for reading and writing files

// represents a individualized unit of work
type Job = Box<dyn FnOnce() + Send + 'static>;

// represents a thread pool
// manages a set of worker threads that can be used to perform individualized units or separations of work
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {

    // create a new thread pool with a given number of worker threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // create sender and receiver for the thread pool
        let (sender, receiver): (Sender<Message>, Receiver<Message>) = unbounded();

        // create a mutex to allow multiple threads to access the receiver
        let receiver = Arc::new(Mutex::new(receiver));

        // create a list of new worker threads (size is the number of threads)
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // execute a given function using a thread from the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // wrap the function in a box
        let job = Box::new(f);

        // send the job to a worker thread
        if let Err(error) = self.sender.send(Message::NewJob(job)) {
            println!("thread_pool: was not able to send job to worker: {:?}", error);
        }
    }

    // wait for all threads to finish executing
    pub fn join(&mut self) {
        // loop through all the worker threads
        for worker in &mut self.workers {
            println!("thread_pool: joining worker {} to main thread", worker.id);

            // join the worker thread to the main thread
            // wait for the worker thread to finish executing
            worker.thread.take().unwrap().join().unwrap();
        }
    }

    // terminate all worker threads
    pub fn shutdown(&mut self) {
        // loop through all the worker threads
        for worker in &mut self.workers {
            println!("thread_pool: shutting down worker {}", worker.id);

            // send a terminate message to the worker thread
            self.sender.send(Message::Terminate).unwrap();

            // join the worker thread to the main thread
            // wait for the worker thread to finish executing
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

// represents a message sent to a worker
enum Message {
    NewJob(Job),
    Terminate,
}

// represents a worker thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // create a new worker thread with id and receiver channel
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {

            // lock the receiver and wait for a message
            match receiver.lock().unwrap().recv() {
                Ok(message) => {
                    match message {
                        // add a new job to the thread
                        Message::NewJob(job) => {
                            // println!("Worker {} got a job; executing.", id);
                            job();
                        }

                        // externally terminate the thread
                        Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);

                            break;
                        }
                    }
                }
                Err(error) => {
                    println!("thread_pool: was not able to receive job from worker: {:?}", error);
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}