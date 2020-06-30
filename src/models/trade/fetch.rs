use serde::{Serialize, Deserialize};

use crate::models::Item;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFetchResult {
    pub result: Vec<TradeFetchResultEntry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFetchResultEntry {
    id: String,
    listing: TradeFetchListing,
    item: Item
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFetchListing {
    method: String,
    indexed: String,
    stash: Option<TradeFetchListingStash>,
    whisper: String,
    account: TradeFetchListingAccount,
    price: TradeFetchListingPrice
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFetchListingStash {
    name: String,
    x: u32,
    y: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeFetchListingAccount {
    name: String,
    last_character_name: String,
    online: TradeFetchListingAccountOnline,
    language: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFetchListingAccountOnline {
    league: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFetchListingPrice {
    #[serde(rename = "type")]
    price_type: String,
    amount: f32,
    currency: String
}