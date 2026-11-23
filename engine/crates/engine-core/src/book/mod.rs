use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::{
    order::Order,
    types::{MarketId, OrderId, Price},
};

pub mod ops;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Default)]
pub struct OrderBook {
    bids: BTreeMap<Price, VecDeque<OrderId>>,
    asks: BTreeMap<Price, VecDeque<OrderId>>,
    orders: HashMap<OrderId, Order>,
}

#[derive(Debug, Clone, Default)]
pub struct OrderBooks {
    map: BTreeMap<MarketId, OrderBook>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookError {
    InvalidOrder,
    DuplicateOrder,
    OrderNotFound
}