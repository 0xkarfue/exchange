use crate::{
    book::{BookError, OrderBook},
    order::Order,
    types::{OrderId, Price, Quantity, Side, UserId},
};

fn order(id: u64, side: Side, price: u64, qty: u64) -> Order {
    Order {
        id: OrderId(id),
        user_id: UserId(1),
        side,
        price: Price(price),
        qty: Quantity(qty),
        remaining: Quantity(qty),
    }
}

#[test]
fn insert_valid_order() {
    let mut book = OrderBook::default();

    let result = book.insert(order(1, Side::Buy, 100, 5));

    assert_eq!(result, Ok(()));
    assert!(book.get(OrderId(1)).is_some());
}

#[test]
fn insert_duplicate_order() {
    let mut book = OrderBook::default();
    let o = order(1, Side::Buy, 100, 5);

    book.insert(o).unwrap();
    let result = book.insert(o);

    assert_eq!(result, Err(BookError::DuplicateOrder));
}

#[test]
fn best_bid_is_highest() {
    let mut book = OrderBook::default();

    book.insert(order(1, Side::Buy, 99, 1)).unwrap();
    book.insert(order(2, Side::Buy, 100, 1)).unwrap();
    book.insert(order(3, Side::Buy, 98, 1)).unwrap();

    assert_eq!(book.best_bid(), Some(Price(100)));
}

#[test]
fn best_ask_is_lowest() {
    let mut book = OrderBook::default();

    book.insert(order(1, Side::Sell, 103, 1)).unwrap();
    book.insert(order(2, Side::Sell, 101, 1)).unwrap();
    book.insert(order(3, Side::Sell, 105, 1)).unwrap();

    assert_eq!(book.best_ask(), Some(Price(101)));
}

#[test]
fn empty_book_has_no_best_prices() {
    let book = OrderBook::default();

    assert_eq!(book.best_bid(), None);
    assert_eq!(book.best_ask(), None);
}

#[test]
fn get_existing_order() {
    let mut book = OrderBook::default();
    let o = order(1, Side::Buy, 100, 5);

    book.insert(o).unwrap();

    assert_eq!(book.get(OrderId(1)), Some(&o));
}

#[test]
fn get_nonexistent_order() {
    let book = OrderBook::default();

    assert_eq!(book.get(OrderId(42)), None);
}
