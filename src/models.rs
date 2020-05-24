use crate::schema::*;

use diesel::*;
use diesel::serialize::{self, IsNull, Output, ToSql, WriteTuple};
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::*;
use diesel::pg::Pg;
use std::io::Write;

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

#[derive(Queryable, Associations)]
#[belongs_to(Character)]
pub struct StashList {
    pub id: i32,
    pub character_id: i32,
}

#[derive(Queryable, Associations)]
#[belongs_to(StashList)]
pub struct StashTab {
    id: String,
    league: Option<String>,
    public: bool,
    stash_list_id: i32,
    stash_type: String,
    stash: Option<String>,
}

#[derive(Queryable, Associations)]
#[belongs_to(StashTab)]
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
    fractured: Option<bool>,
    fractured_mods: Option<Vec<String>>,
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
    sockets: Option<Vec<Socket>>,
    stack_size: Option<i32>,
    stash_tab_id: String,
    support: Option<bool>,
    talisman_tier: Option<i32>,
    type_line: String,
    utility_mods: Option<Vec<String>>,
    verified: bool,
    w: i32,
    x: Option<i32>,
    y: Option<i32>,
}

#[derive(SqlType, Debug)]
#[postgres(type_name = "item_requirement")]
pub struct ItemRequirementsType;

#[derive(Debug, FromSqlRow, PartialEq, AsExpression)]
#[sql_type = "ItemRequirementsType"]
pub struct ItemRequirements {
    name: String,
    values: Vec<ItemLineContentValue>,
    display_mode: i32,
    prop_type: Option<i32>,
    progress: Option<f32>
}

impl ToSql<ItemRequirements, Pg> for ItemRequirements {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Array<ItemLineContentValueType>, Integer, Nullable<Integer>, Nullable<Float>)>::write_tuple(
            &(self.name.as_str(), self.values.clone(), self.display_mode, self.prop_type, self.progress),
            out,
        )
    }
}

#[derive(SqlType, Debug)]
#[postgres(type_name = "socket_type")]
pub struct SocketType;

#[derive(Debug, FromSqlRow, PartialEq, AsExpression)]
#[sql_type = "SocketType"]
pub struct Socket {
    group: i32,
    attr: String,
    s_colour: String
}

impl ToSql<SocketType, Pg> for Socket {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Integer, Text, Text)>::write_tuple(
            &(self.group, self.attr.as_str(), self.s_colour.as_str()),
            out,
        )
    }
}

impl FromSql<SocketType, Pg> for Socket {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (group, attr, s_colour) =  FromSql::<Record<(Integer, Text, Text)>, Pg>::from_sql(bytes)?;
        Ok(Socket {group, attr, s_colour})
    }
}

#[derive(SqlType, Debug)]
#[postgres(type_name = "influence_type")]
pub struct InfluenceType;

#[derive(Debug, FromSqlRow, PartialEq, AsExpression)]
#[sql_type = "InfluenceType"]
pub struct Influence {
    shaper: Option<bool>,
    elder: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    hunter: Option<bool>,
    warlord: Option<bool>,
}

impl ToSql<InfluenceType, Pg> for Influence {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Nullable<Bool>, Nullable<Bool>, Nullable<Bool>, Nullable<Bool>, Nullable<Bool>, Nullable<Bool>)>::write_tuple(
            &(self.shaper, self.elder, self.crusader, self.redeemer, self.hunter, self.warlord),
            out,
        )
    }
}

impl FromSql<InfluenceType, Pg> for Influence {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (shaper, elder, crusader, redeemer, hunter, warlord) = FromSql::<Record<(Nullable<Bool>, Nullable<Bool>, Nullable<Bool>, Nullable<Bool>, Nullable<Bool>, Nullable<Bool>)>, Pg>::from_sql(bytes)?;
        Ok(Influence {shaper, elder, crusader, redeemer, hunter, warlord})
    }
}

#[derive(SqlType, Debug)]
#[postgres(type_name = "item_line_content_value")]
pub struct ItemLineContentValueType;

#[derive(Debug, FromSqlRow, PartialEq, AsExpression, Clone)]
#[sql_type = "ItemLineContentValueType"]
pub struct ItemLineContentValue(String, i32);

impl ToSql<ItemLineContentValueType, Pg> for ItemLineContentValue {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Integer)>::write_tuple(
            &(self.0.as_str(), self.1),
            out,
        )
    }
}

impl FromSql<ItemLineContentValueType, Pg> for ItemLineContentValue {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (name, value) = FromSql::<Record<(Text, Integer)>, Pg>::from_sql(bytes)?;
        Ok(ItemLineContentValue(name, value))
    }
}

#[derive(SqlType, Debug)]
#[postgres(type_name = "frame_type")]
pub struct FrameTypeType;

#[derive(Debug, FromSqlRow, PartialEq, AsExpression)]
#[sql_type = "FrameTypeType"]
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

impl ToSql<FrameTypeType, Pg> for FrameType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            FrameType::Normal => out.write_all(b"Normal")?,
            FrameType::Magic => out.write_all(b"Magic")?,
            FrameType::Rare => out.write_all(b"Rare")?,
            FrameType::Unique => out.write_all(b"Unique")?,
            FrameType::Gem => out.write_all(b"Gem")?,
            FrameType::Currency => out.write_all(b"Currency")?,
            FrameType::DivinationCard => out.write_all(b"DivinationCard")?,
            FrameType::QuestItem => out.write_all(b"QuestItem")?,
            FrameType::Prophecy => out.write_all(b"Prophecy")?,
            FrameType::Relic => out.write_all(b"Relic")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FrameTypeType, Pg> for FrameType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Normal" => Ok(FrameType::Normal),
            b"Magic" => Ok(FrameType::Magic),
            b"Rare" => Ok(FrameType::Rare),
            b"Unique" => Ok(FrameType::Unique),
            b"Gem" => Ok(FrameType::Gem),
            b"Currency" => Ok(FrameType::Currency),
            b"DivinationCard" => Ok(FrameType::DivinationCard),
            b"QuestItem" =>Ok(FrameType::QuestItem),
            b"Prophecy" => Ok(FrameType::Prophecy),
            b"Relic" => Ok(FrameType::Relic),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
#[derive(SqlType)]
#[postgres(type_name = "item_extended_data")]
pub struct ItemExtendedDataType;

#[derive(Debug, FromSqlRow, PartialEq, AsExpression)]
pub struct ItemExtendedData {
    category: String,
    subcategories: Option<Vec<String>>,
    prefixes: Option<i32>,
    suffixes: Option<i32>,
    base_type: Option<String>
}

impl ToSql<ItemExtendedDataType, Pg> for ItemExtendedData {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Nullable<Array<Text>>, Nullable<Integer>, Nullable<Integer>, Nullable<Text>)>::write_tuple(
            &(self.category.as_str(), self.subcategories.clone(), self.prefixes, self.suffixes, self.base_type.clone()),
            out,
        )
    }
}

impl FromSql<ItemExtendedDataType, Pg> for ItemExtendedData {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (category, subcategories, prefixes, suffixes, base_type) = FromSql::<Record<(Text, Nullable<Array<Text>>, Nullable<Integer>, Nullable<Integer>, Nullable<Text>)>, Pg>::from_sql(bytes)?;
        Ok(ItemExtendedData {category, subcategories, prefixes, suffixes, base_type})
    }
}