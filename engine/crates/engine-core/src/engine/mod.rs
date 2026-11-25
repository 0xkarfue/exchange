use std::collections::BTreeMap;

use crate::{
    balance::Balances,
    book::OrderBooks,
    market::{Asset, Market},
    types::{AssetId, MarketId},
};

pub mod ops;
pub struct Engine {
    orderbooks: OrderBooks,
    balances: Balances,
    markets: BTreeMap<MarketId, Market>,
    assets: BTreeMap<AssetId, Asset>,
    order_id_counter: u64,
    event_sequence_counter: u64,
}
