use diesel::*;
use diesel::pg::upsert::*;

use crate::schema::*;
use crate::models::{TableStashTab, Item};
use crate::models::{ItemRequirements, ItemExtendedData, FrameType, Influence, Socket};

#[derive(Clone, Queryable, Associations, Insertable, AsChangeset)]
#[belongs_to(TableStashTab, foreign_key = "stash_tab_id")]
#[table_name = "items"]
pub struct TableItem {
    pub abyss_jewel: Option<bool>,
    pub additional_properties: Option<Vec<ItemRequirements>>,
    pub art_file_name: Option<String>,
    pub category: String,
    pub corrupted: Option<bool>,
    pub cosmetic_mods: Option<Vec<String>>,
    pub crafted_mods: Option<Vec<String>>,
    pub descr_text: Option<String>,
    pub dubplicated: Option<bool>,
    pub elder: Option<bool>,
    pub enchant_mods: Option<Vec<String>>,
    pub explicit_mods: Option<Vec<String>>,
    pub extended: ItemExtendedData,
    pub flavour_text: Option<Vec<String>>,
    pub fractured: Option<bool>,
    pub fractured_mods: Option<Vec<String>>,
    pub frame_type: FrameType,
    pub h: i32,
    pub icon: String,
    pub id: String,
    pub identified: bool,
    pub ilvl: i32,
    pub implicit_mods: Option<Vec<String>>,
    pub influences: Option<Influence>,
    pub inventory_id: Option<String>,
    pub is_relic: Option<bool>,
    pub league: String,
    pub locked_to_character: Option<bool>,
    pub max_stack_size: Option<i32>,
    pub name: String,
    pub next_level_requirements: Option<Vec<ItemRequirements>>,
    pub note: Option<String>,
    pub properties: Option<Vec<ItemRequirements>>,
    pub prophecy_diff_text: Option<String>,
    pub prophecy_text: Option<String>,
    pub requirements: Option<Vec<ItemRequirements>>,
    pub sec_descr_text: Option<String>,
    pub shaper: Option<bool>,
    pub sockets: Option<Vec<Socket>>,
    pub stack_size: Option<i32>,
    pub stash_tab_id: String,
    pub support: Option<bool>,
    pub talisman_tier: Option<i32>,
    pub type_line: String,
    pub utility_mods: Option<Vec<String>>,
    pub verified: bool,
    pub w: i32,
    pub x: i32,
    pub y: i32,
}

impl TableItem {
    pub fn upsert_item(conn: &PgConnection, new_stash_tab_id: String, item: Item) -> TableItem {
        use crate::schema::items::dsl::*;
    
        let new_item = item.convert_to_table_item(new_stash_tab_id);
    
        diesel::insert_into(items)
            .values(new_item.clone())
            .on_conflict(id)
            .do_update()
            .set(new_item)
            .get_result(conn)
            .expect("Could not create new item")
    }

    pub fn upsert_items(conn: &PgConnection, new_stash_tab_id: String, item_to_insert: Vec<Item>) {
        use crate::schema::items::dsl::*;
        
        let mut new_items: Vec<TableItem> = Vec::new();

        for item in item_to_insert {
            new_items.push(item.convert_to_table_item(new_stash_tab_id.clone()));
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