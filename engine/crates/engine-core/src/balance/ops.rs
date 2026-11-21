use crate::{
    balance::{Balance, BalanceError, Balances},
    types::{Amount, AssetId, UserId},
};

impl Balances {
    pub fn get(&self, user: UserId, asset: AssetId) -> Balance {
        self.map.get(&(user, asset)).copied().unwrap_or_default()
    }

    pub fn deposit(
        &mut self,
        user: UserId,
        asset: AssetId,
        amount: Amount,
    ) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }
        let mut bal = self.get(user, asset);
        bal.available = bal
            .available
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;
        self.map.insert((user, asset), bal);
        Ok(())
    }

    pub fn withdraw(
        &mut self,
        user: UserId,
        asset: AssetId,
        amount: Amount,
    ) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }
        let mut bal = self.get(user, asset);
        bal.available = bal
            .available
            .checked_sub(amount)
            .ok_or(BalanceError::Insufficient)?;
        self.map.insert((user, asset), bal);
        Ok(())
    }

    pub fn lock(
        &mut self,
        user: UserId,
        asset: AssetId,
        amount: Amount,
    ) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }
        let mut bal = self.get(user, asset);
        bal.available = bal
            .available
            .checked_sub(amount)
            .ok_or(BalanceError::Insufficient)?;
        bal.locked = bal
            .locked
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;
        self.map.insert((user, asset), bal);
        Ok(())
    }

    pub fn unlock(
        &mut self,
        user: UserId,
        asset: AssetId,
        amount: Amount,
    ) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }
        let mut bal = self.get(user, asset);
        bal.locked = bal
            .locked
            .checked_sub(amount)
            .ok_or(BalanceError::Insufficient)?;
        bal.available = bal
            .available
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;
        self.map.insert((user, asset), bal);
        Ok(())
    }

    pub fn transfer_locked(
        &mut self,
        from: UserId,
        to: UserId,
        asset: AssetId,
        amount: Amount,
    ) -> Result<(), BalanceError> {
        if amount == 0 {
            return Err(BalanceError::InvalidAmount);
        }
        if from == to {
            return self.unlock(from, asset, amount);
        }

        let mut sender = self.get(from, asset);
        let mut receiver = self.get(to, asset);

        sender.locked = sender
            .locked
            .checked_sub(amount)
            .ok_or(BalanceError::Insufficient)?;
        receiver.available = receiver
            .available
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;

        self.map.insert((from, asset), sender);
        self.map.insert((to, asset), receiver);
        Ok(())
    }
}
