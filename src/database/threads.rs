use std::{sync::{Arc, Mutex}, thread};

use crossbeam::channel::{Sender, unbounded, Receiver};

// setup thread pooling for reading and writing files

// represents a individualized unit of work
type Job = Box<dyn FnOnce() + Send + 'static>;

// represents a thread pool
// manages a set of threads that can be used to perform individualized units or separations of work
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {

    // create a new thread pool with a given number of worker threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // create sender and receiver for the channel
        let (sender, receiver): (Sender<Message>, Receiver<Message>) = unbounded();

        // create a mutex wrapped in a smart pointer to allow multiple threads to access the receiver
        let receiver = Arc::new(Mutex::new(receiver));

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
        let job = Box::new(f);

        if let Err(error) = self.sender.send(Message::NewJob(job)) {
            println!("thread_pool: was not able to send job to worker: {:?}", error);
        }
    }

    // wait for all threads to finish executing
    pub fn join(&mut self) {
        for worker in &mut self.workers {
            println!("thread_pool: hutting down worker {}", worker.id);

            self.sender.send(Message::Terminate).unwrap();
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

//

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
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}