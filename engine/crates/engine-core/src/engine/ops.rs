use std::collections::HashMap;

use crate::{
    balance::Balances, book::OrderBooks, command::Command, engine::Engine, event::Event,
    market::Market,
};

impl Engine {
    pub fn new(markets: Vec<Market>) -> Self {
        let market_map = markets
            .into_iter()
            .map(|market| (market.id, market))
            .collect::<HashMap<_, _>>();

        let orderbooks = OrderBooks::new(market_map.keys().copied());

        Self {
            orderbooks,
            balances: Balances::default(),
            markets: market_map,
            order_id_counter: 1,
            event_sequence_counter: 1,
        }
    }

    pub fn apply(&mut self, command: Command) -> Vec<Event> {
        match command {
            Command::Deposit(cmd) => {
                println!("Deposit: {:?}", cmd);
            }

            Command::Withdraw(cmd) => {
                println!("Withdraw: {:?}", cmd);
            }

            Command::PlaceLimit(cmd) => {
                println!("PlaceLimit: {:?}", cmd);
            }
            Command::Cancel(cmd) => {
                println!("Cancel: {:?}", cmd);
            }
        }
        todo!()
    }
}
