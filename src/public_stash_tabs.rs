extern crate serde_repr;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
pub struct PublicStashTabRequest {
    pub next_change_id: String,
    stashes: Vec<StashTab>
}

#[derive(Deserialize, Debug)]
pub struct StashTab {
    id: String,
    public: bool,
    accountName: Option<String>,
    lastCharacterName: Option<String>,
    stash: Option<String>,
    stashType: Option<String>,
    league: Option<String>,
    items: Option<Vec<Item>>
}

#[derive(Deserialize, Debug)]
pub struct Item {
    verified: bool,
    w: u16,
    h: u16,
    ilvl: u16,
    icon: String,
    league: String,
    id: String,
    elder: Option<bool>,
    shaper: Option<bool>,
    fractured: Option<bool>,
    delve: Option<bool>,
    sockets: Option<Vec<Socket>>,
    name: String,
    typeLine: String,
    identified: bool,
    corrupted: Option<bool>,
    requirements: Option<Vec<ItemRequirements>>, // Check into 25494418-26606951-25417312-25066282-26588786 stash 400
    properties: Option<Vec<ItemLineContent>>,
    talismanTier: Option<u16>,
    utilityMods: Option<Vec<String>>,
    implicitMods: Option<Vec<String>>,
    explicitMods: Option<Vec<String>>,
    craftedMods: Option<Vec<String>>,
    enchantMods: Option<Vec<String>>,
    fracturedMods: Option<Vec<String>>,
    flavourText: Option<Vec<String>>,
    descrText: Option<String>,
    secDescrText: Option<String>,
    frameType: FrameType,
    extended: Option<ItemExtendedData>, // Replaces category https://www.pathofexile.com/forum/view-thread/2627531
    category: Option<Vec<String>>,
    x: Option<u16>,
    y: Option<u16>,
    inventoryId: Option<String>,
    isRelic: Option<bool>,
    socketedItems: Option<Vec<Item>>,
    socket: Option<u16>,
    colour: Option<String>
}

#[derive(Deserialize, Debug)]
struct Socket {
    group: u16,
    attr: StatAttribute,
    sColour: SColour
}

#[derive(Deserialize, Debug)]
struct ItemRequirements {

}

#[derive(Deserialize, Debug)]
enum StatAttribute {
    D, // DEX
    I, // INT
    S, // STR
    G,
    A,
    DV
}

#[derive(Deserialize, Debug)]
enum SColour {
    B,
    G,
    R,
    W,
    A,
    DV
}

#[derive(Deserialize, Debug)]
struct ItemLineContent {
    name: String,
    values: Vec<ItemLineContentValue>,
    displayMode: DisplayMode
}

#[derive(Deserialize, Debug)]
struct ItemLineContentValue(String, u16);

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum DisplayMode {
    DefaultName = 0,
    ValueName = 1,
    ProgressName = 2,
    NthName = 3
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum FrameType {
    Normal = 0,
    Magic = 1,
    Rare = 2,
    Unique = 3,
    Gem = 4,
    UNKNOWN = 5,
    UNKNOWN2 = 6,
    UNKNOWN3 = 7,
    UNKNOWN4 = 8,
    UNKNOWN5 = 9
}

// https://www.pathofexile.com/forum/view-forum/674
#[derive(Deserialize, Debug)]
struct ItemExtendedData {
    category: String,
    subcategories: Option<Vec<String>>,
    prefixes: Option<u16>,
    suffixes: Option<u16>
}