#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod public_stash_tabs;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Account, NewAccount};

pub fn create_account<'a>(conn: &PgConnection, name: &'a str, last_character: &'a str) -> Account{
    use schema::accounts;

    let new_account = NewAccount {
        name: name,
        last_character: last_character
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Could not create a new Account")
}

pub fn lookup_account<'a>(conn: &PgConnection, search_name: &'a str) -> Result<Account, diesel::result::Error> {
    use schema::accounts::dsl::*;

    accounts.filter(name.eq(search_name))
        .first::<Account>(conn)
}

pub fn update_account(conn: &PgConnection, account: Account) -> Result<Account, diesel::result::Error> {
    use schema::accounts::dsl::*;

    diesel::update(accounts.filter(id.eq(account.id)))
        .set(&account)
        .get_result::<Account>(conn)
}

use self::models::TableStashTab;
use self::public_stash_tabs::StashTab;

pub fn upsert_stash(conn: &PgConnection, new_account_id: i32, stash_tab: StashTab) -> Result<TableStashTab, diesel::result::Error> {
    use schema::stash_tabs::dsl::*;

    let new_stash_tab = stash_tab.convertToTableStashTab(new_account_id);

    diesel::insert_into(stash_tabs)
        .values(new_stash_tab.clone())
        .on_conflict(id)
        .do_update()
        .set(new_stash_tab)
        .get_result::<TableStashTab>(conn)
}

use self::models::TableItem;
use self::public_stash_tabs::Item;

pub fn update_item(conn: &PgConnection, new_stash_tab_id: String, item: Item) -> TableItem {
    use schema::items::dsl::*;

    let new_item = item.convertToTableItem(new_stash_tab_id);

    diesel::insert_into(items)
        .values(new_item.clone())
        .on_conflict(id)
        .do_update()
        .set(new_item)
        .get_result(conn)
        .expect("Could not create new item")
}