use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::models::{Account, StashTab};

use elasticsearch::http::transport::*;
use elasticsearch::*;
use tokio::task;

struct WorkQueue<T: Send> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send> WorkQueue<T> {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new()))
        }
    }

    fn get_work(&self) -> Option<T> {
        let maybe_queue = self.queue.lock();

        if let Ok(mut queue) = maybe_queue {
            queue.pop_front()
        } else {
            panic!("Rip!")
        }
    }

    fn add_work(&self, work: T) -> usize {
        if let Ok(mut queue) = self.queue.lock() {
            queue.push_back(work);

            queue.len()
        } else {
            panic!("Rip!")
        }
    }

    fn get_size(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    work_queue: Arc<WorkQueue<StashTab>>,
}

impl ThreadPool {
    pub fn new(size: usize, con_pool: Arc<Elasticsearch>) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let work_queue: Arc<WorkQueue<StashTab>> = Arc::new(WorkQueue::new());

        for id in 0..size {
            let new_client = Arc::clone(&con_pool);
            workers.push(Worker::new(id, Arc::clone(&work_queue), new_client));
        }

        ThreadPool {
            workers,
            work_queue,
        }
    }

    pub fn send_work(&self, work: StashTab) {
        let queue_length = self.work_queue.add_work(work);
        println!("Current queue length: {}", queue_length);

    }

    pub fn get_size(&self) -> usize {
        self.work_queue.get_size()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, work_queue: Arc<WorkQueue<StashTab>>, client: Arc<Elasticsearch>) -> Worker {
        let thread = thread::spawn( move || {
            loop {
                let work = work_queue.get_work();

                match work {
                    Some(work) => {
                        let process_time = std::time::Instant::now();
                        println!("Thread {} got work: {}", id, work.id);
                        update_stash(Arc::clone(&client), work.clone());
                        println!("Thread {} finished work: {} taking {}ms", id, work.id, process_time.elapsed().as_millis());

                    },
                    _ => {
                        //println!("Could not get work!");
                        thread::sleep(std::time::Duration::from_millis(100));
                    },
                }
            }
        });

        Worker {
            id, 
            thread: Some(thread)
        }
    }
}

fn update_stash(client: Arc<Elasticsearch>, stash_tab: StashTab) {
    // Lookup account
    
    // Insert stash

    // Insert items

}