
// Serde
use serde::{Serialize, Deserialize};
use serde_repr::Deserialize_repr;
use serde_with::skip_serializing_none;

#[derive(Deserialize, Debug)]
pub struct PublicStashTabRequest {
    pub next_change_id: Option<String>,
    pub stashes: Option<Vec<StashTab>>
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct StashTab {
    pub account_name: Option<String>,
    pub id: String,   
    pub items: Option<Vec<Item>>,
    pub last_character_name: Option<String>,
    pub league: Option<String>,
    pub public: bool,
    pub stash_type: String,
    pub stash: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct Item {
    abyss_jewel: Option<bool>,
    additional_properties: Option<Vec<ItemRequirements>>,
    art_file_name: Option<String>,
    category: String,
    corrupted: Option<bool>,
    cosmetic_mods: Option<Vec<String>>,
    crafted_mods: Option<Vec<String>>,
    descr_text: Option<String>,
    dubplicated: Option<bool>,
    elder: Option<bool>,
    enchant_mods: Option<Vec<String>>,
    explicit_mods: Option<Vec<String>>,
    extended: ItemExtendedData,
    flavour_text: Option<Vec<String>>,
    fractured_mods: Option<Vec<String>>,
    fractured: Option<bool>,
    frame_type: FrameType,
    h: i32,
    icon: String,
    id: String,
    identified: bool,
    ilvl: i32,
    implicit_mods: Option<Vec<String>>,
    influences: Option<Influence>,
    inventory_id: Option<String>,
    is_relic: Option<bool>,
    league: String,
    locked_to_character: Option<bool>,
    max_stack_size: Option<i32>,
    name: String,
    next_level_requirements: Option<Vec<ItemRequirements>>,
    note: Option<String>,
    properties: Option<Vec<ItemRequirements>>,
    prophecy_diff_text: Option<String>,
    prophecy_text: Option<String>,
    requirements: Option<Vec<ItemRequirements>>,
    sec_descr_text: Option<String>,
    shaper: Option<bool>,
    socketed_items: Option<Vec<Item>>,
    sockets: Option<Vec<Socket>>,
    stack_size: Option<i32>,
    support: Option<bool>,
    talisman_tier: Option<i32>,
    type_line: String,
    utility_mods: Option<Vec<String>>,
    verified: bool,
    w: i32,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Socket {
    group: i32,
    attr: String,
    s_colour: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Influence {
    shaper: Option<bool>,
    elder: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    hunter: Option<bool>,
    warlord: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ItemRequirements {
    name: String,
    values: Vec<ItemLineContentValue>,
    display_mode: i32,
    #[serde(alias = "type")]
    prop_type: Option<i32>,
    progress: Option<f32>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemLineContentValue(String, i32);

#[derive(Serialize, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum FrameType {
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

impl Default for FrameType {
    fn default() -> Self {
        FrameType::Normal
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ItemExtendedData {
    category: String,
    subcategories: Option<Vec<String>>,
    prefixes: Option<i32>,
    suffixes: Option<i32>,
    base_type: Option<String>
}