use crate::types::{Amount, AssetId, MarketId, OrderId, Price, Quantity, Side, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deposit {
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Withdraw {
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceLimit {
    pub user_id: UserId,
    pub market_id: MarketId,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cancel {
    pub user_id: UserId,
    pub market_id: MarketId,
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Deposit(Deposit),
    Withdraw(Withdraw),
    PlaceLimit(PlaceLimit),
    Cancel(Cancel),
}