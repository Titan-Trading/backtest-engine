use super::{bar_map::BarMap, candlestick::Candlestick};


// represents a bar of candlesticks linked by timestamp across multiple symbols and exchanges
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Bar {
    pub timestamp: i64,

    // map of exchange and symbol to candlestick
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
    // example source id would be "binance:BTCUSDT"
    pub fn add_candlestick(&mut self, source_id: String, candlestick: Candlestick) {
        let key = source_id.split(":").collect::<Vec<&str>>();
        let exchange = key[0].to_string();
        let symbol = key[1].to_string();
        self.candlesticks.insert((exchange, symbol), candlestick);
    }

    // check if the bar has a candlestick for the source id
    pub fn has_candlestick(&self, source_id: String) -> bool {
        let key = source_id.split(":").collect::<Vec<&str>>();
        let exchange = key[0].to_string();
        let symbol = key[1].to_string();
        self.candlesticks.bars.contains_key(&(exchange, symbol))
    }
}