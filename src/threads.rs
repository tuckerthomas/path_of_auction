use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::models::{Account, TableItem, TableStashTab, StashTab};
use crate::PgPool;

use diesel::prelude::*;

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
    pub fn new(size: usize, con_pool: Arc<PgPool>) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let work_queue: Arc<WorkQueue<StashTab>> = Arc::new(WorkQueue::new());

        for id in 0..size {
            let new_pool = Arc::clone(&con_pool);
            workers.push(Worker::new(id, Arc::clone(&work_queue), new_pool));
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
    fn new(id: usize, work_queue: Arc<WorkQueue<StashTab>>, db_pool: Arc<PgPool>) -> Worker {
        let thread = thread::spawn(move || {
            let conn = db_pool.get().unwrap();

            loop {
                let work = work_queue.get_work();

                match work {
                    Some(work) => {
                        let process_time = std::time::Instant::now();
                        //println!("Thread {} got work: {}", id, work.id);
                        update_stash(&conn, work.clone());
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

fn update_stash(conn: &PgConnection, stash_tab: StashTab) {
    // Lookup account
    let account = match Account::lookup_account(conn, stash_tab.account_name.clone().unwrap().as_str()) {
        Ok(mut account) => {
            // Update last character
            account.last_character = stash_tab.last_character_name.clone().unwrap();
            account = match Account::update_account(conn, account.clone()) {
                Err(e) => {
                    eprintln!("Could not update account {}", e);
                    account
                }
                Ok(updated_account) => updated_account,
            };
            account
        },
        Err(_e) => {
            // TODO: break into different errors
            // Error doesn't get used, because account didnt exist
            Account::create_account(conn, stash_tab.account_name.clone().unwrap().as_str(), stash_tab.last_character_name.clone().unwrap().as_str())
        }
    };
    
    // Insert stash
    let stash = match TableStashTab::upsert_stash(conn, account.id, stash_tab.clone()) {
        Ok(stash) => stash, 
        Err(e) => { 
            eprintln!("Could not update stash {}", e);
            stash_tab.clone().convert_to_table_stash_tab(account.id)
        }
    };

    /*
    // insert items
    for item in stash_tab.items.unwrap() {
        TableItem::upsert_item(conn, stash.id.clone(), item);
    }*/

    TableItem::upsert_items(conn, stash.id.clone(), stash_tab.items.unwrap());
}