use crate::types::{Amount, AssetId, AssetSymbol, MarketId, Price, Quantity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    pub id: AssetId,
    pub symbol: AssetSymbol,
    pub decimals: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Market {
    pub id: MarketId,
    pub base_asset: AssetId,
    pub quote_asset: AssetId,
    pub tick_size: Price,
    pub lot_size: Quantity,
    pub min_qty: Quantity,
    pub min_notional: Amount,
}
