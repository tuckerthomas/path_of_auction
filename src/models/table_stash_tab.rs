use diesel::*;

use crate::models::Account;
use crate::models::StashTab;
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

impl TableStashTab {
    pub fn upsert_stash(conn: &PgConnection, new_account_id: i32, stash_tab: StashTab) -> Result<TableStashTab, diesel::result::Error> {
        use crate::schema::stash_tabs::dsl::*;
    
        let new_stash_tab = stash_tab.convert_to_table_stash_tab(new_account_id);
    
        diesel::insert_into(stash_tabs)
            .values(new_stash_tab.clone())
            .on_conflict(id)
            .do_update()
            .set(new_stash_tab)
            .get_result::<TableStashTab>(conn)
    }
}