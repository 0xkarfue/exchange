use crate::types::{OrderId, Price, Quantity, Side, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub user: UserId,
    pub side: Side,
    pub price: Price,
    pub qty: Quantity,
    pub remaining: Quantity,
}