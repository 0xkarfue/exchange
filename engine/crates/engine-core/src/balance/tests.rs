use crate::{
    balance::{self, Balance, Balances},
    types::Amount,
};

#[test]
fn get_balance_good() {
    let mut balances = Balances::default();

    balances.map.insert(
        (1, 1),
        Balance {
            available: 100,
            locked: 0,
        },
    );

    assert_eq!(
        balances.get(1, 1),
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
        (1, 1),
        Balance {
            available: 100,
            locked: 0,
        },
    );

    assert_eq!(balances.get(1, 2), Balance::default());
}

#[test]
fn deposit_balance_good() {
    let mut balances = Balances::default();
    let amount: Amount = 100;

    let result = balances.deposit(1, 1, amount);

    assert_eq!(result, Ok(()));

    assert_eq!(
        balances.get(1, 1),
        Balance {
            available: 100,
            locked: 0,
        }
    );
}

#[test]
fn deposit_balance_multiple() {}

#[test]
fn deposit_balance_overflow() {}

#[test]
fn withdraw_balance_good() {}

#[test]
fn withdraw_balance_insufficient() {}

#[test]
fn withdraw_balance_no_change_on_failure() {}

#[test]
fn lock_balance_good() {}

#[test]
fn lock_balance_insufficient() {}

#[test]
fn lock_balance_overflow() {}

#[test]
fn unlock_balance_good() {}

#[test]
fn unlock_balance_insufficient() {}

#[test]
fn unlock_balance_overflow() {}

#[test]
fn transfer_locked_balance_good() {}

#[test]
fn transfer_locked_balance_insufficient() {}

#[test]
fn transfer_locked_balance_overflow() {}

#[test]
fn transfer_locked_balance_same_user() {}
