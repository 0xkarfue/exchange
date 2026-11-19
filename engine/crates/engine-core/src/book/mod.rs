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
    pub bids: BTreeMap<Price, VecDeque<OrderId>>,
    pub asks: BTreeMap<Price, VecDeque<OrderId>>, 
    pub orders: HashMap<OrderId, Order>, 
}

#[derive(Debug, Clone, Default)]
pub struct OrderBooks {
    pub map: BTreeMap<MarketId, OrderBook>,
}