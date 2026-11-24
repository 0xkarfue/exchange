use crate::types::{Amount, AssetId, MarketId, OrderId, OrderStatus, Price, Quantity, Side, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sequence(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    UnkownMarket,
    InvalidPrice,
    InvalidQuantity,
    BelowMinNotional,
    ZeroAmount,
    InsufficientBalance,
    OrderNotFound,
    NotOrderOwner,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BalanceDeposited {
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BalanceWithdrawn {
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FundsLocked {
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FundsUnlocked {
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockedTransferred {
    pub from_user: UserId,
    pub to_user: UserId,
    pub asset_id: AssetId,
    pub amount: Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderPlaced {
    pub order_id: OrderId,
    pub user_id: UserId,
    pub market_id: MarketId,
    pub side: Side,
    pub price: Price,
    pub qty: Quantity,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trade {
    pub trade_id: TradeId,
    pub market_id: MarketId,
    pub price: Price,
    pub qty: Quantity,
    pub maker_order: OrderId,
    pub maker_user: UserId,
    pub taker_order: OrderId,
    pub taker_user: UserId,
    pub taker_side: Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderUpdated {
    pub order_id: OrderId,
    pub user_id: UserId,
    pub market_id: MarketId,
    pub status: OrderStatus,
    pub remaining: Quantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderCancelled {
    pub order_id: OrderId,
    pub user_id: UserId,
    pub market_id: MarketId,
    pub remaining: Quantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rejected {
    pub user_id: UserId,
    pub reason: RejectReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    BalanceDeposited(BalanceDeposited),
    BalanceWithdrawn(BalanceWithdrawn),
    FundsLocked(FundsLocked),
    FundsUnlocked(FundsUnlocked),
    LockedTransferred(LockedTransferred),
    OrderPlaced(OrderPlaced),
    Trade(Trade),
    OrderUpdated(OrderUpdated),
    OrderCancelled(OrderCancelled),
    Rejected(Rejected),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequencedEvent {
    pub seq: Sequence,
    pub event: Event,
}