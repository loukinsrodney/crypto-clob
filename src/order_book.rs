use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: BTreeMap<f64, f64>,  // price -> quantity
    pub asks: BTreeMap<f64, f64>,  // price -> quantity
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, side: &str, price: f64, quantity: f64) {
        let book = if side == "bids" { &mut self.bids } else { &mut self.asks };

        if quantity == 0.0 {
            book.remove(&price);
        } else {
            book.insert(price, quantity);
        }
    }

    pub fn top_of_book(&self) -> (Option<(f64, f64)>, Option<(f64, f64)>) {
        (
            self.bids.iter().rev().next().map(|(&price, &qty)| (price, qty)),
            self.asks.iter().next().map(|(&price, &qty)| (price, qty)),
        )
    }
}