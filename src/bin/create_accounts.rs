extern crate path_of_auction;
extern crate diesel;

use self::path_of_auction::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like to name the account?");
    let mut name = String::new();

    stdin().read_line(&mut name).unwrap();

    let name = &name[..(name.len() - 1)]; // Remove newline

    let account = create_account(&connection, &name, &name);
    println!("\nCreated Account {} with id {}", account.name, account.id);
}