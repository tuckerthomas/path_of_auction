use crate::schema::*;
use crate::public_stash_tabs::*;

use diesel::*;

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Associations)]
#[belongs_to(Account)]
pub struct Character {
    pub id: i32,
    pub account_id: i32,
    pub name: String,

}

#[derive(Insertable)]
#[table_name = "characters"]
pub struct NewCharacter<'a> {
    pub account_id: i32,
    pub name: &'a str,
}

#[derive(Queryable, Associations)]
#[belongs_to(Character)]
pub struct StashList {
    pub id: i32,
    pub character_id: i32,
}
#[derive(Insertable)]
#[table_name = "stash_lists"]
pub struct NewStashList {
    pub character_id: i32,
}

#[derive(Queryable, Associations, Insertable)]
#[belongs_to(StashList)]
#[table_name = "stash_tabs"]
pub struct TableStashTab {
    pub id: String,
    pub stash_list_id: i32,
    pub public: bool,
    pub stash: Option<String>,
    pub stash_type: String,
    pub league: Option<String>
}

#[derive(Queryable, Associations, Insertable)]
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