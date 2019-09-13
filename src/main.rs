extern crate hyper;

use hyper::Client;
use futures_util::try_stream::TryStreamExt;

mod public_stash_tabs;
use public_stash_tabs::PublicStashTabRequest;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

//type Result<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[tokio::main]
async fn main() -> Result<()> {
    // Last ID left on 25494418-26606951-25417312-25066282-26588786
    let inital_id = "482693335-499240894-471409853-539142114-512304116".to_string();
    

    let now = std::time::Instant::now();
    let pub_stash_tab = get_stash_tabs(inital_id).await?;
    println!("Full Deserialize in {}ms", now.elapsed().as_millis());

    /*
    println!("Got first request!");

    let mut done = false;

    let half_second = std::time::Duration::from_millis(500);
    */

    /*
    while !done {
        std::thread::sleep(half_second);
        let mut now = std::time::Instant::now();
        // Insert Get Request
        println!("Response in {}ms", now.elapsed().as_millis());
        now = std::time::Instant::now();

        println!("Body len: {}", body.len());

        let new_stash_tabs_list: public_stash_tabs::PublicStashTabRequest =
            serde_json::from_str(&body)?;
        println!("Deserialize in {}ms", now.elapsed().as_millis());

        println!(
            "Got new request!, ID={}", new_stash_tabs_list.next_change_id);
    };
    */
    Ok(())
}

async fn get_stash_tabs(next_id: String) -> Result<PublicStashTabRequest> {
    let public_stash_tabs_api = "http://api.pathofexile.com/public-stash-tabs".to_string();

    let mut concat_pub_stash_tab_api = "".to_string();
    if next_id.is_empty() {
        concat_pub_stash_tab_api = public_stash_tabs_api
    } else {
        concat_pub_stash_tab_api = format!("{}?id={}", public_stash_tabs_api, next_id)
    }

    let public_stash_tab_api_full = concat_pub_stash_tab_api.parse().unwrap();

    let client = Client::new();

    let mut now = std::time::Instant::now();
    let res = client.get(public_stash_tab_api_full).await?;
    println!("Response in {}ms", now.elapsed().as_millis());

    now = std::time::Instant::now();
    let body = res.into_body().try_concat().await?;
    println!("Deserialize in {}ms", now.elapsed().as_millis());

    let pub_stash_tab = serde_json::from_slice(&body)?;

    Ok(pub_stash_tab)
}

// Multiple Error Type Handling
enum ParseError {
    SerdeError(serde_json::Error),
    HyperError(hyper::Error),
}

impl From<hyper::Error> for ParseError {
    fn from(error: hyper::Error) -> Self {
        ParseError::HyperError(error)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(error: serde_json::Error) -> Self {
        ParseError::SerdeError(error)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseError::SerdeError(ref err) => write!(f, "Serde Error: {}", err),
            ParseError::HyperError(ref err) => write!(f, "Hyper Error: {}", err),
        }
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseError::SerdeError(ref err) => write!(f, "Serde Error: {}", err),
            ParseError::HyperError(ref err) => write!(f, "Hyper Error: {}", err),
        }
    }
}
