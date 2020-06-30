use serde::{Serialize, Deserialize};
use hyper::{Uri, Request, Method, Body};

pub enum TradeSearchRequest {
    TradeUri
}

impl TradeSearchRequest {
    pub fn new(query: &'static str) -> Request<Body> {
        let trade_uri: Uri = TradeSearchRequest::TradeUri.as_str().parse().unwrap();
        Request::builder()
            .method(Method::POST)
            .uri(trade_uri)
            .header("content-type", "application/json")
            .body(Body::from(query)).unwrap()
    }

    pub fn as_str(&self) -> &str {
        match self {
            &TradeSearchRequest::TradeUri => "http://www.pathofexile.com/api/trade/search/Standard"
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchQuery {
    status: TradeSearchQueryStatus,
    name: String,
    #[serde(rename = "type")]
    query_type: String,
    stats: Vec<TradeSearchStat>,
    filters: TradeSearchFilters,
    sort: TradeSearchSort
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchQueryStatus {
    option: String
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchStat {
    #[serde(rename = "type")]
    stat_type: String,
    filters: Vec<TradeSearchStatFilter>
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchStatFilter {
    id: String,
    value: TradeSearchStatValue,
    disabled: bool
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchStatValue {
    min: u32,
    max: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchFilters {
    weapon_filters: TradeSearchWeaponFilter,
    armour_filters: TradeSearchArmourFilter,
    socket_filters: TradeSearchSocketFilter,
    req_filters: TradeSearchReqFilter,
    misc_filters: TradeSearchMiscFilter,
    trade_filters: TradeSearchFilter,
    type_filters: TradeSearchTypeFilter
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchWeaponFilter {
    disabled: bool,
    filters: TradeSearchWeaponFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchWeaponFilterFilters {
    damage: TradeSearchWeaponFilterValue,
    crit: TradeSearchWeaponFilterValue,
    aps: TradeSearchWeaponFilterValue,
    dps: TradeSearchWeaponFilterValue,
    edps: TradeSearchWeaponFilterValue,
    pdps: TradeSearchWeaponFilterValue
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchWeaponFilterValue {
    min: u32,
    max: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchArmourFilter {
    disabled: bool,
    filters: TradeSearchArmourFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchArmourFilterFilters {
    ar: TradeSearchArmourFilterValue,
    es: TradeSearchArmourFilterValue,
    ev: TradeSearchArmourFilterValue,
    block: TradeSearchArmourFilterValue
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchArmourFilterValue {
    min: u32,
    max: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchSocketFilter {
    disabled: bool,
    filters: TradeSearchSocketFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchSocketFilterFilters {
    sockets: TradeSearchSocketFilterValue,
    links: TradeSearchSocketFilterValue
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchSocketFilterValue {
    min: u32,
    max: u32,
    r: u32,
    g: u32,
    b: u32,
    w: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchReqFilter {
    disabled: bool,
    filters: TradeSearchReqFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchReqFilterFilters {
    lvl: TradeSearchReqFilterValue,
    dex: TradeSearchReqFilterValue,
    #[serde(rename = "str")]
    req_str: TradeSearchReqFilterValue,
    int: TradeSearchReqFilterValue,
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchReqFilterValue {
    min: u32,
    max: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchMiscFilter {
    disabled: bool,
    filters: TradeSearchMiscFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchMiscFilterFilters {
    quality: TradeSearchMiscFilterValue,
    map_tier: TradeSearchMiscFilterValue,
    map_iiq: TradeSearchMiscFilterValue,
    gem_level: TradeSearchMiscFilterValue,
    ilvl: TradeSearchMiscFilterValue,
    map_packsize: TradeSearchMiscFilterValue,
    map_iir: TradeSearchMiscFilterValue,
    talisman_tier: TradeSearchMiscFilterValue,
    alternate_art: TradeSearchMiscFilterOption,
    identified: TradeSearchMiscFilterOption,
    corrupted: TradeSearchMiscFilterOption,
    crafted: TradeSearchMiscFilterOption,
    enchanted: TradeSearchMiscFilterOption
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchMiscFilterValue {
    min: u32,
    max: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchMiscFilterOption {
    option: String
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchFilter {
    disabled: bool,
    filters: TradeSearchFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchFilterFilters {
    account: TradeSearchFilterInput,
    sale_type: TradeSearchFilterOption,
    price: TradeSearchFilterValue
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchFilterInput {
    input: String
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchFilterOption {
    option: String
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchFilterValue {
    min: u32,
    max: u32
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchTypeFilter {
    filters: TradeSearchTypeFilterFilters
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchTypeFilterFilters {
    category: TradeSearchTypeFilterOption,
    rarity: TradeSearchTypeFilterOption
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchTypeFilterOption {
    option: String
}

#[derive(Serialize, Deserialize)]
pub struct TradeSearchSort {
    price: String
}

// TradeSearch Query Result

#[derive(Serialize, Deserialize)]
pub struct TradeSearchResult {
    pub id: String,
    pub complexity: u32,
    pub result: Vec<String>,
    pub total: u32
}