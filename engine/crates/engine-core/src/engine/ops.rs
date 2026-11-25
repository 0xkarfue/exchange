use std::collections::BTreeMap;

use crate::{
    balance::{
        BalanceError::{self},
        Balances,
    },
    book::OrderBooks,
    command::{Cancel, Command, Deposit, PlaceLimit, Withdraw},
    engine::Engine,
    event::{
        self, BalanceDeposited, BalanceWithdrawn,
        Event::{self},
        RejectReason, Rejected, Sequence, SequencedEvent,
    },
    market::{Asset, Market},
    types::{OrderId, Price, Quantity, Side},
};

impl Engine {
    pub fn new(markets: Vec<Market>, assets: Vec<Asset>) -> Self {
        let market_map = markets
            .into_iter()
            .map(|market| (market.id, market))
            .collect::<BTreeMap<_, _>>();

        let asset_map = assets
            .into_iter()
            .map(|asset| (asset.id, asset))
            .collect::<BTreeMap<_, _>>();

        let orderbooks = OrderBooks::new(market_map.keys().copied());

        Self {
            orderbooks,
            balances: Balances::default(),
            markets: market_map,
            assets: asset_map,
            order_id_counter: 1,
            event_sequence_counter: 1,
        }
    }

    pub fn apply(&mut self, command: Command) -> Vec<SequencedEvent> {
        match command {
            Command::Deposit(_cmd) => self.deposit(_cmd),
            Command::Withdraw(_cmd) => self.withdraw(_cmd),
            Command::PlaceLimit(_cmd) => self.place_limit(_cmd),
            Command::Cancel(_cmd) => self.cancel(_cmd),
        }
    }

    fn emit(&mut self, event: Event) -> SequencedEvent {
        let seq = Sequence(self.event_sequence_counter);
        self.event_sequence_counter += 1;
        SequencedEvent { seq, event }
    }

    fn deposit(&mut self, _cmd: Deposit) -> Vec<SequencedEvent> {
        if _cmd.amount == 0 {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::ZeroAmount,
            });
            return vec![self.emit(event)];
        }

        if !self.assets.contains_key(&_cmd.asset_id) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::UnknownAsset,
            });
            return vec![self.emit(event)];
        }

        let result = self
            .balances
            .deposit(_cmd.user_id, _cmd.asset_id, _cmd.amount);

        match result {
            Ok(()) => {
                let event = Event::BalanceDeposited(BalanceDeposited {
                    user_id: _cmd.user_id,
                    asset_id: _cmd.asset_id,
                    amount: _cmd.amount,
                });

                return vec![self.emit(event)];
            }

            Err(BalanceError::Overflow) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::Overflow,
                });

                return vec![self.emit(event)];
            }
            Err(BalanceError::InvalidAmount) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::ZeroAmount,
                });

                return vec![self.emit(event)];
            }
            Err(BalanceError::Insufficient) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::InsufficientBalance,
                });

                return vec![self.emit(event)];
            }
        }
    }

    fn withdraw(&mut self, _cmd: Withdraw) -> Vec<SequencedEvent> {
        if _cmd.amount == 0 {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::ZeroAmount,
            });
            return vec![self.emit(event)];
        }

        if !self.assets.contains_key(&_cmd.asset_id) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::UnknownAsset,
            });
            return vec![self.emit(event)];
        }

        let result = self
            .balances
            .withdraw(_cmd.user_id, _cmd.asset_id, _cmd.amount);

        match result {
            Ok(()) => {
                let event = Event::BalanceWithdrawn(BalanceWithdrawn {
                    user_id: _cmd.user_id,
                    asset_id: _cmd.asset_id,
                    amount: _cmd.amount,
                });
                return vec![self.emit(event)];
            }
            Err(BalanceError::Overflow) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::Overflow,
                });
                return vec![self.emit(event)];
            }
            Err(BalanceError::Insufficient) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::InsufficientBalance,
                });
                return vec![self.emit(event)];
            }
            Err(BalanceError::InvalidAmount) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::InvalidPrice,
                });
                return vec![self.emit(event)];
            }
        }
    }

    fn place_limit(&mut self, _cmd: PlaceLimit) -> Vec<SequencedEvent> {
        if !self.markets.contains_key(&_cmd.market_id) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::UnknownMarket,
            });
            return vec![self.emit(event)];
        }

        if _cmd.price == Price(0) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::InvalidPrice,
            });

            return vec![self.emit(event)];
        }

        if _cmd.quantity == Quantity(0) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::InvalidQuantity,
            });

            return vec![self.emit(event)];
        }

        let market = self.markets.get(&_cmd.market_id).unwrap().clone();

        if _cmd.price.0 % market.tick_size.0 != 0 {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::InvalidPrice,
            });
            return vec![self.emit(event)];
        }

        if _cmd.quantity.0 % market.lot_size.0 != 0 {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::InvalidQuantity,
            });
            return vec![self.emit(event)];
        }

        if _cmd.quantity.0 < market.min_qty.0 {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::InvalidQuantity,
            });
            return vec![self.emit(event)];
        }

        let base_asset = self.assets.get(&market.base_asset).unwrap();

        let notional = (_cmd.price.0 as u128)
            .checked_mul(_cmd.quantity.0 as u128)
            .and_then(|value| {
                10u128
                    .checked_pow(base_asset.decimals as u32)
                    .and_then(|divisor| value.checked_div(divisor))
            });

        let Some(notional) = notional else {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::Overflow,
            });
            return vec![self.emit(event)];
        };

        if notional < market.min_notional as u128 {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::BelowMinNotional,
            });
            return vec![self.emit(event)];
        }

        match _cmd.side {
            Side::Buy => {
                let result = self
                    .balances
                    .lock(_cmd.user_id, market.quote_asset, notional);
                match result {
                    Ok(()) => {
                        let event = Event::FundsLocked(event::FundsLocked {
                            user_id: _cmd.user_id,
                            asset_id: market.quote_asset,
                            amount: notional,
                            order_id: OrderId(self.order_id_counter),
                        });
                        self.order_id_counter += 1;
                        return vec![self.emit(event)];
                    }
                    Err(BalanceError::Overflow) => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::Overflow,
                        });
                        return vec![self.emit(event)];
                    }
                    Err(BalanceError::Insufficient) => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::InsufficientBalance,
                        });
                        return vec![self.emit(event)];
                    }
                    Err(BalanceError::InvalidAmount) => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::InvalidPrice,
                        });
                        return vec![self.emit(event)];
                    }
                }
            }
            Side::Sell => {
                let result = self
                    .balances
                    .lock(_cmd.user_id, market.base_asset, notional);
                match result {
                    Ok(()) => {
                        let event = Event::FundsLocked(event::FundsLocked {
                            user_id: _cmd.user_id,
                            asset_id: market.base_asset,
                            amount: notional,
                            order_id: OrderId(self.order_id_counter),
                        });
                        self.order_id_counter += 1;

                        return vec![self.emit(event)];
                    }
                    Err(BalanceError::Overflow) => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::Overflow,
                        });
                        return vec![self.emit(event)];
                    }
                    Err(BalanceError::Insufficient) => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::InsufficientBalance,
                        });
                        return vec![self.emit(event)];
                    }
                    Err(BalanceError::InvalidAmount) => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::InvalidPrice,
                        });
                        return vec![self.emit(event)];
                    }
                }
            }
        }
        //  emit event OrderPlaced, then FundsLocked.
    }

    fn cancel(&mut self, _cmd: Cancel) -> Vec<SequencedEvent> {
        if !self.markets.contains_key(&_cmd.market_id) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::UnknownMarket,
            });
            return vec![self.emit(event)];
        }
        let market = self.markets.get(&_cmd.market_id).unwrap().clone();

        let book = match self.orderbooks.get_mut(_cmd.market_id) {
            Some(book) => book,
            None => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::OrderNotFound,
                });
                return vec![self.emit(event)];
            }
        };

        let order = match book.get(_cmd.order_id) {
            Some(order) => order,
            None => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::OrderNotFound,
                });
                return vec![self.emit(event)];
            }
        };

        if order.user_id != _cmd.user_id {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::NotOrderOwner,
            });
            return vec![self.emit(event)];
        }

        let side = order.side;
        let price = order.price;
        let remaining = order.remaining;

        let (asset_id, refund) = match side {
            Side::Buy => {
                let base_asset = self.assets.get(&market.base_asset).unwrap();

                let refund = (price.0 as u128)
                    .checked_mul(remaining.0 as u128)
                    .and_then(|value| {
                        10u128
                            .checked_pow(base_asset.decimals as u32)
                            .and_then(|divisor| value.checked_div(divisor))
                    });

                let refund = match refund {
                    Some(value) => value,
                    None => {
                        let event = Event::Rejected(Rejected {
                            user_id: _cmd.user_id,
                            reason: RejectReason::Overflow,
                        });
                        return vec![self.emit(event)];
                    }
                };

                (market.quote_asset, refund)
            }

            Side::Sell => (market.base_asset, remaining.0 as u128),
        };

        if let Err(_) = self.balances.unlock(_cmd.user_id, asset_id, refund) {
            let event = Event::Rejected(Rejected {
                user_id: _cmd.user_id,
                reason: RejectReason::Overflow,
            });
            return vec![self.emit(event)];
        }

        let removed_order = match book.cancel(_cmd.order_id) {
            Ok(order) => order,
            Err(_) => {
                let event = Event::Rejected(Rejected {
                    user_id: _cmd.user_id,
                    reason: RejectReason::OrderNotFound,
                });
                return vec![self.emit(event)];
            }
        };

        let cancelled = Event::OrderCancelled(event::OrderCancelled {
            user_id: _cmd.user_id,
            order_id: _cmd.order_id,
            market_id: _cmd.market_id,
            remaining: removed_order.remaining,
        });

        let unlocked = Event::FundsUnlocked(event::FundsUnlocked {
            user_id: _cmd.user_id,
            asset_id,
            amount: refund,
            order_id: _cmd.order_id,
        });

        vec![self.emit(cancelled), self.emit(unlocked)]
    }
}
