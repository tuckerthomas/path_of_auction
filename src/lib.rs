pub mod models;
pub mod threads;

use dotenv::dotenv;
use elasticsearch::http::transport::*;
use elasticsearch::*;

use std::env;
use std::sync::Arc;

pub fn establish_connection(num_threads: u32) -> Arc<Elasticsearch> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set").parse().unwrap();
    let conn_pool = SingleNodeConnectionPool::new(database_url);
    let transport = TransportBuilder::new(conn_pool).build().unwrap();
    Arc::new(Elasticsearch::new(transport))
}