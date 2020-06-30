use bytes::buf::BufExt as _;
use hyper::{Request, Method, Client, Body};

use path_of_auction::models::trade::fetch::*;
use path_of_auction::models::trade::search::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
        let now = std::time::Instant::now();

        let info = 
        r#"{
                "query": {
                        "status": {
                                "option": "online"
                        },
                        "name": "Exalted Orb"
                },
                "sort": {
                        "price": "asc"
                }
        }"#;

        // Create client for http request
        let client = Client::new();
        let res = client.request(TradeSearchRequest::new(info)).await?;

        let body = hyper::body::to_bytes(res).await?;
        
        //println!("{:?}", body);
        
        let search_result_json: serde_json::Value = serde_json::from_reader(body.clone().reader()).unwrap();
        println!("Trade search json {}", search_result_json);
        
        let search_result: TradeSearchResult = serde_json::from_reader(body.reader()).unwrap();

        println!("Trade search in {} ms", now.elapsed().as_millis());

        let mut fetch_uri_base = "http://www.pathofexile.com/api/trade/fetch/".to_owned();

        let mut item_id_iter = search_result.result.iter().peekable();

        let mut count = 1;

        while let Some(item_id) = item_id_iter.next() {
                if item_id_iter.peek().is_some() {
                        fetch_uri_base.push_str(&item_id);
                        fetch_uri_base.push_str(",");
                } else {
                        break;
                }

                count += 1;

                 // Max is 10 tabs apparently
                if count > 9 {
                        break;
                }
        }

        // Dont forget the last one!
        fetch_uri_base.push_str(&search_result.result[search_result.result.len() - 1]);
        
        fetch_uri_base.push_str("?id=");
        fetch_uri_base.push_str(&search_result.id.to_string());

        println!("Checking uri: {}", fetch_uri_base);

        let fetch_uri: hyper::Uri = fetch_uri_base.parse().unwrap();

        let req = Request::builder()
                .method(Method::GET)
                .uri(fetch_uri)
                .body(Body::empty())
                .unwrap();

        let res = client.request(req).await?;
        let body = hyper::body::to_bytes(res).await?;
        
        //println!("{:?}", body);

        let item_result: TradeFetchResult = serde_json::from_reader(body.reader()).unwrap();

        println!("Returned {} items", item_result.result.len());
        println!("Full search in {} ms", now.elapsed().as_millis());

        Ok(())
}