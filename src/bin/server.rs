use bytes::buf::BufExt as _;
use hyper::Client;
use dotenv::dotenv;

use path_of_auction::models::*;
use path_of_auction::threads::*;
use path_of_auction::*;

use std::error::Error;
use std::convert::TryInto;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let nthreads = env::var("NTHREADS").expect("NTHREADS must be set!").parse::<u32>().expect("NTHREADS must be a number!");
    println!("Number of worker threads set to {}!", nthreads);

    // START OF BLIGHT: 475841770-492541661-464580457-531662984-505125106
    // Last ID left on 25494418-26606951-25417312-25066282-26588786
    // START 2947-5227-4265-5439-1849
    // Delerium 715190373-729521754-696364788-787219679-751860877
    let start_id = env::var("START_TAB").expect("START_TAB must be set!");
    
    // Connect to database
    let db_pool = establish_connection(nthreads);

    // Start worker pool
    let work_pool = threads::ThreadPool::new(nthreads.try_into().unwrap(), db_pool);

    let pub_stash_tab = get_stash_tabs(start_id, &work_pool).await?;

    println!("Got first request!");

    let mut next_change_id = pub_stash_tab.next_change_id.unwrap().clone();

    // Acquire next stash tab
    loop {   
        let pub_stash_tab = get_stash_tabs(next_change_id, &work_pool).await?;
        next_change_id = pub_stash_tab.next_change_id.unwrap().clone();
    }
}

async fn get_stash_tabs(next_id: String, work_pool: &ThreadPool) -> Result<PublicStashTabRequest, Box<dyn Error>> {
    //TODO: Track averages
    println!("\nGetting ID: {}", next_id);

    // Setup URI
    let request = PubStashTabRequest::new(next_id.as_str());

    // Setup timers for diagnostics
    let full_req = std::time::Instant::now();
    let mut now = std::time::Instant::now();

    // Create client for http request
    let client = Client::new();
    let res = client.request(request).await?;
    println!("Response in {}ms", now.elapsed().as_millis());

    // Reset now
    now = std::time::Instant::now();

    // TODO: Track total byte processing
    let body = hyper::body::to_bytes(res).await?;
    println!("Byte Conversion in {}ms, {} bytes processed", now.elapsed().as_millis(), body.len());

    // Reset now
    now = std::time::Instant::now();

    // Deserialize request
    let pub_stash_tab: PublicStashTabRequest = serde_json::from_reader(body.clone().reader())?;
    let value: serde_json::Value = serde_json::from_reader(body.reader())?;
    println!("Json?: {}", value);
    println!("Deserialize in {}ms", now.elapsed().as_millis());

    // Reset now
    now = std::time::Instant::now();

    // Check to see if past rate-limiting
    // TODO: Change to read dynamic xratelimit header
    // TODO: un-hard code miliseconds
    if pub_stash_tab.next_change_id.clone().unwrap() == next_id {
        println!("No Data change, sleeping...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    } else {
        for stash_tab in pub_stash_tab.stashes.clone().unwrap() {
            if stash_tab.public {
                // TODO: Determine what happens if all the worker's are busy
                // IE: The work pool sends a message, but there's no thread able to receive it?
                // TODO: Move stash_queue to its own thread, create a mutex for said queue and then send them to the workers
                // TODO: halt thread update until all stashes have been processed in the queue
                work_pool.send_work(stash_tab);
            }
        }
    }

    // Clear out the work queue for the next requeust
    while work_pool.get_size() > 15000 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Processing in {}ms", now.elapsed().as_millis());
    println!("Full Request in {}ms", full_req.elapsed().as_millis());

    if full_req.elapsed().as_millis() < 1000 {
        std::thread::sleep(std::time::Duration::from_millis(((1000 - full_req.elapsed().as_millis())).try_into().unwrap()));
    }

    Ok(pub_stash_tab)
}