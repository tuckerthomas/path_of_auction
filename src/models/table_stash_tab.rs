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
    pub items: Option<Vec<String>>,
    pub stash_data: Option<StashTab>
}

impl TableStashTab {
    pub fn new(new_account_id: i32, stash_tab: StashTab) -> Self {
        let mut item_ids: Vec<String> = Vec::new();

        for item in stash_tab.items.clone().unwrap() {
            item_ids.push(item.id);
        }

        TableStashTab {
            id: stash_tab.id.clone(),
            account_id: new_account_id,
            items: Some(item_ids),
            stash_data: Some(stash_tab)
        }
    }

    pub fn upsert_stash(conn: &PgConnection, new_account_id: i32, stash_tab: StashTab) -> Result<TableStashTab, diesel::result::Error> {
        use crate::schema::stash_tabs::dsl::*;

        let new_stash_tab = TableStashTab::new(new_account_id, stash_tab);
    
        diesel::insert_into(stash_tabs)
            .values(new_stash_tab.clone())
            .on_conflict(id)
            .do_update()
            .set(new_stash_tab)
            .get_result::<TableStashTab>(conn)
    }
}