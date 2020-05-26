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

impl Account {
    pub fn create_account<'a>(conn: &PgConnection, name: &'a str, last_character: &'a str) -> Account{
    
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
        use crate::schema::accounts::dsl::*;
        accounts.filter(name.eq(search_name))
            .first::<Account>(conn)
    }
    
    pub fn update_account(conn: &PgConnection, account: Account) -> Result<Account, diesel::result::Error> {
        use crate::schema::accounts::dsl::*;
    
        diesel::update(accounts.filter(id.eq(account.id)))
            .set(&account)
            .get_result::<Account>(conn)
    }
}