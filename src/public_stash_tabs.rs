extern crate serde_repr;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
pub struct PublicStashTabRequest {
    pub next_change_id: String,
    stashes: Option<Vec<StashTab>>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StashTab {
    id: String,
    public: bool,
    account_name: Option<String>,
    last_character_name: Option<String>,
    stash: Option<String>,
    stash_type: Option<String>,
    items: Option<Vec<Item>>,
    league: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    verified: bool,
    w: u16,
    h: u16,
    ilvl: u16,
    icon: String,
    league: String,
    id: String,
    name: String,
    type_line: String,
    identified: bool,
    extended: ItemExtendedData,
    frame_type: FrameType,
    x: Option<u16>,
    y: Option<u16>,
    note: Option<String>,
    elder: Option<bool>,
    shaper: Option<bool>,
    fractured: Option<bool>,
    dubplicated: Option<bool>,
    sockets: Option<Vec<Socket>>,
    support: Option<bool>,
    corrupted: Option<bool>,
    requirements: Option<Vec<ItemRequirements>>,
    properties: Option<Vec<ItemRequirements>>,
    additional_properties: Option<Vec<ItemRequirements>>,
    next_level_requirements: Option<Vec<ItemRequirements>>,
    talisman_tier: Option<u16>,
    utility_mods: Option<Vec<String>>,
    implicit_mods: Option<Vec<String>>,
    explicit_mods: Option<Vec<String>>,
    crafted_mods: Option<Vec<String>>,
    cosmetic_mods: Option<Vec<String>>,
    enchant_mods: Option<Vec<String>>,
    fractured_mods: Option<Vec<String>>,
    flavour_text: Option<Vec<String>>,
    descr_text: Option<String>,
    sec_descr_text: Option<String>,
    prophecy_diff_text: Option<String>,
    prophecy_text: Option<String>,
    inventory_id: Option<String>,
    is_relic: Option<bool>,
    socketed_items: Option<Vec<Item>>,
    socket: Option<u16>,
    stack_size: Option<u16>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ItemRequirements {
    name: String,
    values: Vec<ItemLineContentValue>,
    display_mode: u16,
    #[serde(alias = "type")]
    prop_type: Option<u16>,
    progress: Option<f32>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Socket {
    group: u16,
    attr: String,
    s_colour: String
}

#[derive(Deserialize, Debug)]
struct ItemLineContentValue(String, u16);

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum FrameType {
    Normal = 0,
    Magic = 1,
    Rare = 2,
    Unique = 3,
    Gem = 4,
    Currency = 5,
    DivinationCard = 6,
    QuestItem = 7,
    Prophecy = 8,
    Relic = 9
}

// https://www.pathofexile.com/forum/view-forum/674
#[derive(Deserialize, Debug)]
struct ItemExtendedData {
    category: String,
    subcategories: Option<Vec<String>>,
    prefixes: Option<u16>,
    suffixes: Option<u16>
}