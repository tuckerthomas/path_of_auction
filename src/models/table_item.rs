use diesel::*;
use diesel::pg::upsert::*;

use crate::schema::*;
use crate::models::{TableStashTab, Item};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Queryable, Associations, Insertable, AsChangeset)]
#[belongs_to(TableStashTab, foreign_key = "stash_tab_id")]
#[table_name = "items"]
pub struct TableItem {
    pub id: String,
    pub stash_tab_id: String,
    pub item_data: Option<Item>
}

impl TableItem {
    pub fn new(stash_tab_id: String, item: Item) -> Self {
        TableItem {
            id: item.id.clone(),
            stash_tab_id: stash_tab_id,
            item_data: Some(item)
        }
    }

    pub fn upsert_item(conn: &PgConnection, new_stash_tab_id: String, item: Item) -> TableItem {
        use crate::schema::items::dsl::*;

        let table_item = TableItem::new(new_stash_tab_id, item);
    
        diesel::insert_into(items)
            .values(table_item.clone())
            .on_conflict(item_data)
            .do_update()
            .set(table_item)
            .get_result(conn)
            .expect("Could not create new item")
    }

    pub fn upsert_items(conn: &PgConnection, new_stash_tab_id: String, item_to_insert: Vec<Item>) {
        use crate::schema::items::dsl::*;
        
        let mut new_items: Vec<TableItem> = Vec::new();

        for item in item_to_insert {
            new_items.push(TableItem::new(new_stash_tab_id.clone(), item));
        }
    
        diesel::insert_into(items)
            .values(&new_items)
            .on_conflict(id)
            .do_update()
            .set(stash_tab_id.eq(excluded(stash_tab_id)))
            .execute(conn)
            .expect("Could not insert items");
    }
}