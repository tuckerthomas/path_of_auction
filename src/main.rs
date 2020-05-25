use bytes::buf::BufExt as _;
use hyper::Client;

use path_of_auction::public_stash_tabs::*;
use path_of_auction::*;

use std::error::Error;
use std::convert::TryInto;

use diesel::prelude::*; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: move to arg/class/config/env_var
    // API Stats https://poe.watch/stats?type=time
    // START OF BLIGHT: 475841770-492541661-464580457-531662984-505125106
    // Last ID left on 25494418-26606951-25417312-25066282-26588786
    // START 2947-5227-4265-5439-1849
    // Delerium 715190373-729521754-696364788-787219679-751860877
    let inital_id = "715487020-729827007-696655000-787533836-752338242".to_string();

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
        std::thread::sleep(std::time::Duration::from_millis(500));
    } else {
        let conn = establish_connection();

        for stash_tab in pub_stash_tab.stashes.clone().unwrap() {
            if stash_tab.public {
                update_stash(&conn, stash_tab);
            }
        }
    }
    println!("Processing in {}ms", now.elapsed().as_millis());
    println!("Full Request in {}ms", full_req.elapsed().as_millis());

    Ok(pub_stash_tab)
}

fn update_stash(conn: &PgConnection, stash_tab: StashTab) {
    // Lookup account
    let account = match lookup_account(conn, stash_tab.account_name.clone().unwrap().as_str()) {
        Ok(mut account) => {
            // Update last character
            account.last_character = stash_tab.last_character_name.unwrap();
            account = match update_account(conn, account.clone()) {
                Err(e) => {
                    eprintln!("Could not update account {}", e);
                    account
                }
                Ok(updated_account) => updated_account,
            };
            account
        },
        Err(e) => {
            eprintln!("Account didn't exist {}", e);
            create_account(conn, stash_tab.account_name.clone().unwrap().as_str(), stash_tab.last_character_name.unwrap().clone().as_str())
        }
    };
    
    // Insert stash
    // insert items
}