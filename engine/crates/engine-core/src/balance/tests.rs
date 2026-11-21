use crate::{
    balance::{Balance, Balances},
    types::{Amount, AssetId, UserId},
};

const USER_1: UserId = UserId(1);
const USER_2: UserId = UserId(2);
const ASSET_1: AssetId = AssetId(1);
const ASSET_2: AssetId = AssetId(2);

#[test]
fn get_balance_good() {
    let mut balances = Balances::default();

    balances.map.insert(
        (USER_1, ASSET_1),
        Balance {
            available: 100,
            locked: 0,
        },
    );

    assert_eq!(
        balances.get(USER_1, ASSET_1),
        Balance {
            available: 100,
            locked: 0,
        }
    );
}

#[test]
fn get_balance_bad() {
    let mut balances = Balances::default();

    balances.map.insert(
        (USER_1, ASSET_1),
        Balance {
            available: 100,
            locked: 0,
        },
    );

    assert_eq!(balances.get(USER_1, ASSET_2), Balance::default());
}

#[test]
fn deposit_balance_good() {
    let mut balances = Balances::default();
    let amount: Amount = 100;

    let result = balances.deposit(USER_1, ASSET_1, amount);

    assert_eq!(result, Ok(()));

    assert_eq!(
        balances.get(USER_1, ASSET_1),
        Balance {
            available: 100,
            locked: 0,
        }
    );
}

#[test]
fn deposit_balance_multiple() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 100).unwrap();
    balances.deposit(USER_1, ASSET_1, 50).unwrap();

    assert_eq!(balances.get(USER_1, ASSET_1).available, 150);
}

#[test]
fn deposit_balance_overflow() {
    let mut balances = Balances::default();
    let amount = Amount::MAX;

    balances.deposit(USER_1, ASSET_1, amount).unwrap();
    let result = balances.deposit(USER_1, ASSET_1, 1);

    assert_eq!(result, Err(crate::balance::BalanceError::Overflow));
    assert_eq!(balances.get(USER_1, ASSET_1).available, Amount::MAX);
}

#[test]
fn withdraw_balance_good() {
    let mut balances = Balances::default();
    let amount = 200;

    balances.deposit(USER_1, ASSET_1, amount).unwrap();

    let result = balances.withdraw(USER_1, ASSET_1, 100);

    assert_eq!(result, Ok(()));
    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
}

#[test]
fn withdraw_balance_insufficient() {
    let mut balances = Balances::default();
    let amount = 200;

    balances.deposit(USER_1, ASSET_1, amount).unwrap();

    let result = balances.withdraw(USER_1, ASSET_1, 500);

    assert_eq!(result, Err(crate::balance::BalanceError::Insufficient));
    assert_eq!(balances.get(USER_1, ASSET_1).available, 200);
}

#[test]
fn lock_balance_good() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 100).unwrap();
    balances.lock(USER_1, ASSET_1, 50).unwrap();

    assert_eq!(balances.get(USER_1, ASSET_1).locked, 50);
}

#[test]
fn lock_balance_insufficient() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 100).unwrap();
    let result = balances.lock(USER_1, ASSET_1, 200);

    assert_eq!(result, Err(crate::balance::BalanceError::Insufficient));
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 0);
    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
}

#[test]
fn lock_balance_overflow() {
    let mut balances = Balances::default();

    balances.map.insert(
        (USER_1, ASSET_1),
        Balance {
            available: 100,
            locked: Amount::MAX,
        },
    );

    let result = balances.lock(USER_1, ASSET_1, 1);

    assert_eq!(result, Err(crate::balance::BalanceError::Overflow));

    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, Amount::MAX);
}

#[test]
fn unlock_balance_good() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 100).unwrap();
    balances.lock(USER_1, ASSET_1, 50).unwrap();

    balances.unlock(USER_1, ASSET_1, 50).unwrap();

    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 0);
}

#[test]
fn unlock_balance_insufficient() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 100).unwrap();
    balances.lock(USER_1, ASSET_1, 50).unwrap();

    let result = balances.unlock(USER_1, ASSET_1, 60);

    assert_eq!(result, Err(crate::balance::BalanceError::Insufficient));
    assert_eq!(balances.get(USER_1, ASSET_1).available, 50);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 50);
}

#[test]
fn unlock_balance_overflow() {
    let mut balances = Balances::default();

    balances.map.insert(
        (USER_1, ASSET_1),
        Balance {
            available: Amount::MAX,
            locked: 1,
        },
    );

    let result = balances.unlock(USER_1, ASSET_1, 1);

    assert_eq!(result, Err(crate::balance::BalanceError::Overflow));
    assert_eq!(balances.get(USER_1, ASSET_1).available, Amount::MAX);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 1);
}

#[test]
fn transfer_locked_balance_good() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 200).unwrap();
    balances.deposit(USER_2, ASSET_1, 300).unwrap();
    balances.lock(USER_1, ASSET_1, 100).unwrap();

    balances
        .transfer_locked(USER_1, USER_2, ASSET_1, 50)
        .unwrap();

    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 50);

    assert_eq!(balances.get(USER_2, ASSET_1).available, 350);
    assert_eq!(balances.get(USER_2, ASSET_1).locked, 0);
}

#[test]
fn transfer_locked_balance_insufficient() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 200).unwrap();
    balances.deposit(USER_2, ASSET_1, 300).unwrap();
    balances.lock(USER_1, ASSET_1, 100).unwrap();

    let result = balances.transfer_locked(USER_1, USER_2, ASSET_1, 150);

    assert_eq!(result, Err(crate::balance::BalanceError::Insufficient));

    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 100);

    assert_eq!(balances.get(USER_2, ASSET_1).available, 300);
    assert_eq!(balances.get(USER_2, ASSET_1).locked, 0);
}

#[test]
fn transfer_locked_balance_overflow() {
    let mut balances = Balances::default();

    balances.map.insert(
        (USER_1, ASSET_1),
        Balance {
            available: 100,
            locked: 50,
        },
    );

    balances.map.insert(
        (USER_2, ASSET_1),
        Balance {
            available: Amount::MAX,
            locked: 0,
        },
    );

    let result = balances.transfer_locked(USER_1, USER_2, ASSET_1, 10);

    assert_eq!(result, Err(crate::balance::BalanceError::Overflow));

    assert_eq!(balances.get(USER_1, ASSET_1).available, 100);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 50);

    assert_eq!(balances.get(USER_2, ASSET_1).available, Amount::MAX);
    assert_eq!(balances.get(USER_2, ASSET_1).locked, 0);
}

#[test]
fn transfer_locked_balance_same_user() {
    let mut balances = Balances::default();

    balances.deposit(USER_1, ASSET_1, 200).unwrap();
    balances.lock(USER_1, ASSET_1, 100).unwrap();

    balances
        .transfer_locked(USER_1, USER_1, ASSET_1, 50)
        .unwrap();

    assert_eq!(balances.get(USER_1, ASSET_1).available, 150);
    assert_eq!(balances.get(USER_1, ASSET_1).locked, 50);
}
