use diesel::*;

use crate::models::Account;
use crate::schema::*;

#[derive(Clone, Queryable, Associations, Insertable, AsChangeset)]
#[belongs_to(Account)]
#[table_name = "stash_tabs"]
pub struct TableStashTab {
    pub id: String,
    pub account_id: i32,
    pub public: bool,
    pub stash: Option<String>,
    pub stash_type: String,
    pub league: Option<String>
}