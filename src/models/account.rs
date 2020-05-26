use diesel::*;

use crate::schema::*;

#[derive(Clone, Queryable, AsChangeset)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub last_character: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub last_character: &'a str,
}