extern crate diesel;
extern crate path_of_auction;

use path_of_auction::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::accounts::dsl::*;

    let connection = establish_connection().get().unwrap();
    let results = accounts
        .limit(5)
        .load::<Account>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for account in results {
        println!("{}", account.id);
    }
}