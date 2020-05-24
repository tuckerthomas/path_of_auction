use serde_with::skip_serializing_none;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
pub struct PublicStashTabRequest {
    pub next_change_id: Option<String>,
    stashes: Option<Vec<StashTab>>
}

#[derive(Default, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct StashTab {
    account_name: Option<String>,
    id: String,   
    items: Option<Vec<Item>>,
    last_character_name: Option<String>,
    league: Option<String>,
    public: bool,
    stash_type: String,
    stash: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Deserialize, Debug)]
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
    h: u16,
    icon: String,
    id: String,
    identified: bool,
    ilvl: u16,
    implicit_mods: Option<Vec<String>>,
    influences: Option<Influence>,
    inventory_id: Option<String>,
    is_relic: Option<bool>,
    league: String,
    locked_to_character: Option<bool>,
    max_stack_size: Option<u16>,
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
    stack_size: Option<u16>,
    support: Option<bool>,
    talisman_tier: Option<u16>,
    type_line: String,
    utility_mods: Option<Vec<String>>,
    verified: bool,
    w: u16,
    x: u16,
    y: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Socket {
    group: u16,
    attr: String,
    s_colour: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Influence {
    shaper: Option<bool>,
    elder: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    hunter: Option<bool>,
    warlord: Option<bool>,
}

#[derive(Default, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase")]
struct ItemRequirements {
    name: String,
    values: Vec<ItemLineContentValue>,
    display_mode: u16,
    #[serde(alias = "type")]
    prop_type: Option<u16>,
    progress: Option<f32>
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

impl Default for FrameType {
    fn default() -> Self {
        FrameType::Normal
    }
}

// https://www.pathofexile.com/forum/view-forum/674
#[derive(Default, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase")]
struct ItemExtendedData {
    category: String,
    subcategories: Option<Vec<String>>,
    prefixes: Option<u16>,
    suffixes: Option<u16>,
    base_type: Option<String>
}