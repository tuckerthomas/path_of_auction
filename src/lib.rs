#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Account, NewAccount};

pub fn create_account<'a>(conn: &PgConnection, name: &'a str) -> Account{
    use schema::accounts;

    let new_account = NewAccount {
        name: name,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Could not create a new Account")
}