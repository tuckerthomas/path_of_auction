
// Serde
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::skip_serializing_none;

// Hyper
use hyper::{Request, Body, Method, Uri};

// Custom composite type creation
use diesel::*;
use diesel::serialize::{Output, ToSql};
use diesel::deserialize::{FromSql};
use diesel::sql_types::*;
use diesel::pg::Pg;
use std::io::Write;

pub enum PubStashTabRequest {
    PubStashTabUri
}

impl PubStashTabRequest {
    pub fn new<'a>(id: &'a str) -> Request<Body> {
        let id_uri = format!("{}?id={}", PubStashTabRequest::PubStashTabUri.as_str(), id);
        let trade_uri: Uri = id_uri.parse().unwrap();
        Request::builder()
            .method(Method::GET)
            .uri(trade_uri)
            .body(Body::empty()).unwrap()
    }

    pub fn as_str(&self) -> &str {
        match self {
            &PubStashTabRequest::PubStashTabUri => "http://api.pathofexile.com/public-stash-tabs"
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PublicStashTabRequest {
    pub next_change_id: Option<String>,
    pub stashes: Option<Vec<StashTab>>
}

#[derive(FromSqlRow, Clone, Default, Debug, serde::Serialize, serde::Deserialize, AsExpression)]
#[serde(default, rename_all = "camelCase")]
#[sql_type = "Jsonb"]
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

impl FromSql<Jsonb, Pg> for StashTab {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for StashTab {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}

#[skip_serializing_none]
#[derive(FromSqlRow, Default, serde::Serialize, serde::Deserialize, AsExpression, Debug, Clone)]
#[serde(default, rename_all = "camelCase")]
#[sql_type = "Jsonb"]
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
    pub id: String,
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

impl FromSql<Jsonb, Pg> for Item {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for Item {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Socket {
    group: i32,
    attr: String,
    s_colour: String
}

#[derive(Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Influence {
    shaper: Option<bool>,
    elder: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    hunter: Option<bool>,
    warlord: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(default, rename_all = "camelCase")]
pub struct ItemRequirements {
    name: String,
    values: Vec<ItemLineContentValue>,
    display_mode: i32,
    #[serde(alias = "type")]
    prop_type: Option<i32>,
    progress: Option<f32>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, AsExpression, Queryable)]
pub struct ItemLineContentValue(String, i32);

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, AsExpression)]
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

#[derive(Default, Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(default)]
pub struct ItemExtendedData {
    category: Option<String>,
    subcategories: Option<Vec<String>>,
    prefixes: Option<i32>,
    suffixes: Option<i32>,
    base_type: Option<String>,
    dps: Option<f32>,
    pdps: Option<f32>,
    edps: Option<f32>,
    dps_aug: Option<bool>,
    edps_aug: Option<bool>,
    mods: Option<ItemExtendedDataMods>,
    text: Option<String>
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(default, rename_all = "camelCase")]
pub struct ItemExtendedDataMods {
    implicit: Vec<ItemExtendedDataModsValues>,
    explicit:Vec<ItemExtendedDataModsValues>
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(default, rename_all = "camelCase")]
pub struct ItemExtendedDataModsValues {
    name: String,
    tier: String,
    magnitudes: Option<Vec<ItemExtendedDataModsValuesMagnitudes>>
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, AsExpression, Queryable)]
#[serde(default, rename_all = "camelCase")]
pub struct ItemExtendedDataModsValuesMagnitudes {
    hash: String,
    min: i32,
    max: i32
}