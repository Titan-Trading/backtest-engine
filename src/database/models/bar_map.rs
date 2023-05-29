use std::{collections::HashMap, hash::{Hash, Hasher}};
use super::candlestick::Candlestick;


// represents a map of a single candlestick by exchange and symbol
#[derive(Clone, Debug, PartialEq)]
pub struct BarMap {
    // map of exchange, symbol and interval to list of candlestick
    pub bars: HashMap<(String, String, String), Candlestick>,
}

impl BarMap {
    pub fn new() -> Self {
        Self {
            bars: HashMap::new(),
        }
    }

    pub fn new_with(bars: HashMap<(String, String, String), Candlestick>) -> Self {
        Self {
            bars,
        }
    }

    pub fn insert(&mut self, key: (String, String, String), candlestick: Candlestick) {
        let key_copy = key.clone();
        let exchange = key.0;
        let symbol = key.1;
        let interval = key.2;
        if !self.bars.contains_key(&key_copy) {
            self.bars.insert(key_copy, candlestick);
        }
    }

    pub fn get(&self, exchange: String, symbol: String, interval: String) -> Option<&Candlestick> {
        self.bars.get(&(exchange, symbol, interval))
    }
}

impl Hash for BarMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sorted_keys: Vec<_> = self.bars.keys().collect();
        sorted_keys.sort();
        for key in sorted_keys {
            key.hash(state);
            self.bars.get(key).unwrap().hash(state);
        }
    }
}