use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStatResult {
    result: Vec<TradeStatEntry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStatEntry {
    label: String,
    entries: Vec<TradeStatEntryProperty>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStatEntryProperty {
    id: String,
    text: String,
    #[serde(rename = "type")]
    prop_type: String,
    option: Option<TradeStatEntryPropertyOption>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStatEntryPropertyOption {
    options: Vec<TradeStatEntryPropertyOptionEntry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStatEntryPropertyOptionEntry {
    id: u32,
    text: String
}