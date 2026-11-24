use crate::{
    book::{BookError, OrderBook, OrderBooks}, order::Order, types::{MarketId, OrderId, Price, Quantity, Side},
};

impl OrderBook {
    pub fn insert(&mut self, order: Order) -> Result<(), BookError> {
        if self.orders.contains_key(&order.id) {
            return Err(BookError::DuplicateOrder);
        }
        if order.remaining == Quantity(0) {
            return Err(BookError::InvalidOrder);
        }
        if order.remaining > order.qty {
            return Err(BookError::InvalidOrder);
        }
        let levels = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        levels.entry(order.price).or_default().push_back(order.id);
        self.orders.insert(order.id, order);

        Ok(())
    }

    pub fn best_bid(&self) -> Option<Price> {
        self.bids.last_key_value().map(|(price, _)| *price)
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.first_key_value().map(|(price, _)| *price)
    }

    pub fn get(&self, id: OrderId) -> Option<&Order> {
        self.orders.get(&id)
    }

    pub fn cancel(&mut self, id: OrderId) -> Result<Order, BookError> {
        let order = self.orders.get(&id).ok_or(BookError::OrderNotFound)?;

        let side = order.side;
        let price = order.price;

        let levels = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        if let Some(queue) = levels.get_mut(&price) {
            queue.retain(|&order_id| order_id != id);

      
            if queue.is_empty() {
                levels.remove(&price);
            }
        }

        let order = self.orders.remove(&id).ok_or(BookError::OrderNotFound)?;

        Ok(order)
    }
}


impl OrderBooks {
    pub fn new(market_ids: impl IntoIterator<Item = MarketId>) -> Self {
        let map = market_ids
            .into_iter()
            .map(|market_id| (market_id, OrderBook::default()))
            .collect();

        Self { map }
    }
}