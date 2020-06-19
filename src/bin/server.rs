use bytes::buf::BufExt as _;
use hyper::Client;
use elasticsearch::*;
use serde_json::Value;

use path_of_auction::models::*;
use path_of_auction::threads::*;
use path_of_auction::*;

use std::error::Error;
use std::convert::TryInto;
use std::sync::Arc;

const NTHREADS: u32 = 200;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to database
    let client = establish_connection(NTHREADS);

    // Start worker pool
    let work_pool = threads::ThreadPool::new(NTHREADS.try_into().unwrap(), Arc::clone(&client));

    // TODO: move to arg/class/config/env_var
    // API Stats https://poe.watch/stats?type=time
    // START OF BLIGHT: 475841770-492541661-464580457-531662984-505125106
    // Last ID left on 25494418-26606951-25417312-25066282-26588786
    // START 2947-5227-4265-5439-1849
    // Delerium 715190373-729521754-696364788-787219679-751860877
    let inital_id = "730821355-744837821-710455127-803797332-767101094".to_string();

    let pub_stash_tab = get_stash_tabs(inital_id, Arc::clone(&client)).await?;

    println!("Got first request!");

    let mut next_change_id = pub_stash_tab.next_change_id.unwrap().clone();

    // Acquire next stash tab
    loop {   
        let pub_stash_tab = get_stash_tabs(next_change_id, Arc::clone(&client)).await?;
        next_change_id = pub_stash_tab.next_change_id.unwrap().clone();
    }
}

async fn get_stash_tabs(next_id: String, e_client: Arc<Elasticsearch>) -> Result<PublicStashTabRequest, Box<dyn Error>> {
    //TODO: Track averages
    println!("\nGetting ID: {}", next_id);

    // Setup URI
    // TODO: move to arg/class/config/env_var
    let public_stash_tab_api_base = "http://api.pathofexile.com/public-stash-tabs";
    let public_stash_tab_api_full = format!("{}?id={}", public_stash_tab_api_base, next_id);
    let public_stash_tab_uri = public_stash_tab_api_full.parse().unwrap();

    // Setup timers for diagnostics
    let full_req = std::time::Instant::now();
    let mut now = std::time::Instant::now();

    // Create client for http request
    let client = Client::new();
    let res = client.get(public_stash_tab_uri).await?;
    println!("Response in {}ms", now.elapsed().as_millis());

    // Reset now
    now = std::time::Instant::now();

    // TODO: Track total byte processing
    let body = hyper::body::to_bytes(res).await?;
    println!("Byte Conversion in {}ms, {} bytes processed", now.elapsed().as_millis(), body.len());

    // Reset now
    now = std::time::Instant::now();

    // Deserialize request
    let pub_stash_tab: PublicStashTabRequest = serde_json::from_reader(body.reader())?;
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
        let mut pub_stash_tabs: Vec<StashTab> = Vec::new();
        
        for stash_tab in pub_stash_tab.stashes.clone().unwrap() {
            if stash_tab.public == true {
                println!("Id: {} added", stash_tab.id);
                pub_stash_tabs.push(stash_tab);
            }
        }

        update_stash(&e_client, pub_stash_tabs.clone()).await;
        println!("Updated {} stashes", pub_stash_tabs.len());
    }

    println!("Processing in {}ms", now.elapsed().as_millis());
    println!("Full Request in {}ms", full_req.elapsed().as_millis());

    if full_req.elapsed().as_millis() < 500 {
        std::thread::sleep(std::time::Duration::from_millis(((500 - full_req.elapsed().as_millis())).try_into().unwrap()));
    }

    Ok(pub_stash_tab)
}

async fn update_stash(client: &Elasticsearch, stash_tabs: Vec<StashTab>) {
    // Lookup account
    
    // Insert stash
    let body: Vec<BulkOperation<_>> = stash_tabs
        .iter()
        .map(|s| {
            let id = s.id.to_string();
            BulkOperation::index(&id, s).into()
        })
        .collect();

    let response = client
        .bulk(BulkParts::Index("stashes"))
        .body(body)
        .send()
        .await.expect("Uhhhhhhhhhhh shits wrong");

    
    let json: Value = response.json().await.unwrap();

    if json["errors"].as_bool().unwrap() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        // TODO: retry failures
        println!("Errors whilst indexing. Failures: {}", failed.len());
    }

    println!("Finished updating");
    // Insert items

}