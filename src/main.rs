
use bytes::buf::BufExt as _;
use hyper::Client;

mod public_stash_tabs;
use public_stash_tabs::PublicStashTabRequest;

use std::error::Error;
use std::convert::TryInto;

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // API Stats https://poe.watch/stats?type=time
    // START OF BLIGHT: 475841770-492541661-464580457-531662984-505125106
    // Last ID left on 25494418-26606951-25417312-25066282-26588786
    // START 2947-5227-4265-5439-1849
    let inital_id = "713783416-728148202-694837269-785769935-750406233".to_string();

    let pub_stash_tab = get_stash_tabs(inital_id).await?;

    println!("Got first request!");

    let mut next_change_id = pub_stash_tab.next_change_id.unwrap().clone();

    // Acquire next stash tab
    loop {    
        let pub_stash_tab = get_stash_tabs(next_change_id).await?;
        next_change_id = pub_stash_tab.next_change_id.unwrap().clone();
    }
}

async fn get_stash_tabs(next_id: String) -> Result<PublicStashTabRequest, Box<dyn Error>> {
    println!("Getting ID: {}", next_id);

    // Setup URI
    let public_stash_tabs_api = "http://api.pathofexile.com/public-stash-tabs".to_string();

    let mut concat_pub_stash_tab_api = "".to_string();
    if next_id.is_empty() {
        println!("Caught up!");
        concat_pub_stash_tab_api = public_stash_tabs_api;
        std::thread::sleep(std::time::Duration::from_millis(100));
    } else {
        concat_pub_stash_tab_api = format!("{}?id={}", public_stash_tabs_api, next_id)
    }

    let public_stash_tab_api_full = concat_pub_stash_tab_api.parse().unwrap();

    // Setup timers for diagnostics
    let full_req = std::time::Instant::now();
    let mut now = std::time::Instant::now();

    // Create client for http request
    let client = Client::new();
    let res = client.get(public_stash_tab_api_full).await?;
    let body = hyper::body::aggregate(res).await?;
    println!("Response in {}ms", now.elapsed().as_millis());

    // Reset now
    now = std::time::Instant::now();

    // Data can be kinda big. Use a buffered reader.
    let buf_body = std::io::BufReader::new(body.reader());

    // Deserialize request
    let pub_stash_tab = serde_json::from_reader(buf_body)?;
    println!("Deserialize in {}ms", now.elapsed().as_millis());
    println!("Full Request in {}ms", full_req.elapsed().as_millis());

    // Check to see if passed rate-limiting
    if full_req.elapsed().as_millis() < 525 {
        let sleep_time = ((525 - full_req.elapsed().as_millis())).try_into().unwrap();
        println!("Acquired Data too quickly, sleeping for {}ms", sleep_time);
        std::thread::sleep(std::time::Duration::from_millis(sleep_time));
    }

    println!("Got stash tab, ID={}\n", next_id);

    Ok(pub_stash_tab)
}