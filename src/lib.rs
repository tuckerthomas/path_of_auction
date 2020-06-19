#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod threads;

use diesel::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use dotenv::dotenv;

use std::env;
use std::sync::Arc;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// TODO:  Figure out why this failes when postgresql isnt runnning
fn init_pool(database_url: &str, num_threads: u32) -> Arc<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Arc::new(Pool::builder().max_size(num_threads + 5).build(manager).unwrap())
}

pub fn establish_connection(num_threads: u32) -> Arc<PgPool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url, num_threads)
}