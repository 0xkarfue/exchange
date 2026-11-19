
use std::collections::BTreeMap;

use crate::types::{Amount, AssetId, UserId};

pub mod ops;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BalanceKey {
    pub user: UserId,
    pub asset: AssetId,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Balance {
    pub available: Amount,
    pub locked: Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError {
    Insufficient,
    Overflow
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AssetBalance {
    pub asset_id: AssetId,
    pub balance: Balance
}


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Balances {
    pub map: BTreeMap<(UserId, AssetId), Balance>,
}