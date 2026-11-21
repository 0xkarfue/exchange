pub type Amount = u128;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UserId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AssetId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderId(pub u64);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetSymbol(pub String);

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
