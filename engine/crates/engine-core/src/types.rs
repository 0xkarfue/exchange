pub type Amount = u128;
pub type Price = u64;
pub type Quantity = u64;
pub type UserId = u64;
pub type AssetId = u64;
pub type MarketId = u64;
pub type OrderId = u64;
pub type AssetSymbol = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}