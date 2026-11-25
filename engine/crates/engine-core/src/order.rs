use crate::types::{OrderId, Price, Quantity, Side, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Order {
    pub id: OrderId,
    pub user_id: UserId,
    pub side: Side,
    pub price: Price,
    pub qty: Quantity,
    pub remaining: Quantity,
}
