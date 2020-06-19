use elasticsearch::*;
use elasticsearch::http::transport::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.0.201:9200".parse()?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    //conn_pool.next();
    let transport = TransportBuilder::new(conn_pool).build()?;
    let client = Elasticsearch::new(transport);

    let response = client
        .index(IndexParts::IndexId("tweets", "1"))
        .body(json!({
            "id": 300
        }))
        .send()
        .await?;

    println!("Successful? {}", response.status_code().is_success());

    let search_response = client
        .search(SearchParts::Index(&["tweets"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "match": {
                    "id": 300
                }
            }
        }))
        .send()
        .await?;

    println!("Results: {}", search_response.text().await?);

    Ok(())
}   