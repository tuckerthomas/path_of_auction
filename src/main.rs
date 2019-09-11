extern crate serde;
extern crate serde_json;

mod public_stash_tabs;

fn main() -> Result<(), ParseError> {
    // Last ID left on 25494418-26606951-25417312-25066282-26588786
    let public_stash_tabs_api = "http://api.pathofexile.com/public-stash-tabs";

    let body = reqwest::get(public_stash_tabs_api)?.text()?;

    println!("Got first request!");

    //let mut error = body.clone();
    //error.truncate(83178);
    //println!("Error point= '{:?}'", error);

    let stash_tabs_list: public_stash_tabs::PublicStashTabRequest = serde_json::from_str(&body)?;
    //println!("body = {:?}", stash_tabs_list);

    let mut done = false;

    let two_seconds = std::time::Duration::from_secs(2);
    let mut public_stash_tabs_api_next_id = format!("{}?id={}", public_stash_tabs_api, stash_tabs_list.next_change_id);

    while !done {
        std::thread::sleep(two_seconds);
        
        let body = reqwest::get(&public_stash_tabs_api_next_id)?.text()?;
        let new_stash_tabs_list: public_stash_tabs::PublicStashTabRequest = serde_json::from_str(&body)?;
        println!("Got new request!, ID={}", new_stash_tabs_list.next_change_id);
        public_stash_tabs_api_next_id = format!("{}?id={}", public_stash_tabs_api, new_stash_tabs_list.next_change_id);
    }

    Ok(())
}

// Multiple Error Type Handling
enum ParseError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error)
}

impl From<reqwest::Error> for ParseError {
    fn from(error: reqwest::Error) -> Self {
        ParseError::ReqwestError(error)
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
            ParseError::ReqwestError(ref err) => write!(f, "Reqwest Error: {}", err),
            ParseError::SerdeError(ref err) => write!(f, "Serde Error: {}", err)
        } 
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseError::ReqwestError(ref err) => write!(f, "Reqwest Error: {}", err),
            ParseError::SerdeError(ref err) => write!(f, "Serde Error: {}", err)
        }
    }
}