use super::{bar_map::BarMap, candlestick::Candlestick};


// represents a bar of candlesticks linked by timestamp across multiple symbols and exchanges
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Bar {
    pub timestamp: i64,

    // map of exchange, symbol, interval to candlestick
    pub candlesticks: BarMap,
}

impl Bar {
    pub fn new(timestamp: i64) -> Self {
        Self {
            timestamp,
            candlesticks: BarMap::new(),
        }
    }

    pub fn new_with(timestamp: i64, candlesticks: BarMap) -> Self {
        Self {
            timestamp,
            candlesticks,
        }
    }

    // add a candlestick to the bar using the source id as the key
    // example key would be "binance:BTCUSDT:1m"
    pub fn add_candlestick(&mut self, key: String, candlestick: Candlestick) {
        let key = key.split(":").collect::<Vec<&str>>();
        let exchange = key[0].to_string();
        let symbol = key[1].to_string();
        let interval = key[2].to_string();
        self.candlesticks.insert((exchange, symbol, interval), candlestick);
    }

    // check if the bar has a candlestick for the source id
    pub fn has_candlestick(&self, key: String) -> bool {
        let key = key.split(":").collect::<Vec<&str>>();
        let exchange = key[0].to_string();
        let symbol = key[1].to_string();
        let interval = key[2].to_string();
        self.candlesticks.bars.contains_key(&(exchange, symbol, interval))
    }
}