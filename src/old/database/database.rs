use std::{io::Error, collections::HashMap};
use super::{engine::{DatabaseEngine, DatabaseEngineType}, query::{Query, QueryResult}};


// Database struct. It is used to represent the database system and all of its clients.
// we support the stmdb file format
// we support the csv file format
// we support the ability to get live data
pub struct Database {
    pub name: String,
    engine: DatabaseEngine,
    clients: Vec<u64>,
}

impl Database {

    // creates a new database
    pub fn new(name: String, engine_type: DatabaseEngineType) -> Database {
        Database {
            name,
            engine: DatabaseEngine::new(engine_type),
            clients: Vec::new(),
        }
    }

    // adds a new client connection to the database
    pub fn connect_client(&mut self, client_id: u64) {
        self.clients.push(client_id);
    }

    // removes a client connection from the database
    pub fn disconnect_client(&mut self, client_id: u64) {
        self.clients.retain(|id| *id != client_id);
    }

    // gets historical data from the database using a query
    pub fn query(&mut self, client_id: u64, query: Query, parameters: HashMap<String, String>) -> Result<QueryResult, Error> {
        Ok(self.engine.query(client_id, query, parameters).unwrap())
    }

    // gets live data from the database using a query
    pub fn stream<F>(&self, client_id: u64, query: Query, on_update: F) -> Result<bool, String>
    where
        F: Fn(Query) -> Result<QueryResult, Error>, {

        Ok(self.engine.stream(client_id, query, on_update).unwrap())
    }

    // inserts data into the database
    pub fn insert(&self, client_id: u64, data: Vec<Bar>) -> Result<bool, Error> {
        Ok(self.engine.insert(client_id, data))
    }
}

// represents a database bar of data (open, high, low, close, volume, timestamp)
#[derive(Debug, Clone)]
pub struct Bar {
    pub symbol: String,
    pub exchange: String,
    pub interval: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: i64,
}

impl Bar {

    // creates a new bar
    pub fn new() -> Bar {
        Bar {
            symbol: "".to_string(),
            exchange: "".to_string(),
            interval: "".to_string(),
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
            timestamp: 0,
        }
    }

    // creates a new bar with data
    pub fn new_with(symbol: String, exchange: String, interval: String, open: f64, high: f64, low: f64, close: f64, volume: f64, timestamp: i64) -> Bar {
        Bar {
            symbol,
            exchange,
            interval,
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        }
    }
}

// represents a set of bars across many symbols and exchanges for the same timestamp
#[derive(Debug, Clone)]
pub struct BarSet {
    pub timestamp: i64,
    pub bars: HashMap<String, Bar>,
}

impl BarSet {

    // creates a new bar set
    pub fn new() -> BarSet {
        BarSet {
            timestamp: 0,
            bars: HashMap::new(),
        }
    }

    // creates a new bar set with data
    pub fn new_with(timestamp: i64, bars: HashMap<String, Bar>) -> BarSet {
        BarSet {
            timestamp,
            bars,
        }
    }
}

// represents a barset that's been consolidated into many other intervals
#[derive(Debug, Clone)]
pub struct ConsolidatedBarSet {
    pub timestamp: i64,

    // bars grouped by a combination of symbol, exchange, and interval
    pub bars: HashMap<String, Bar>,
}

impl ConsolidatedBarSet {

    // creates a new consolidated bar set
    pub fn new() -> ConsolidatedBarSet {
        ConsolidatedBarSet {
            timestamp: 0,
            bars: HashMap::new(),
        }
    }

    // creates a new consolidated bar set with data
    pub fn new_with(timestamp: i64, bars: HashMap<String, Bar>) -> ConsolidatedBarSet {
        ConsolidatedBarSet {
            timestamp,
            bars,
        }
    }
}

// represents a symbol
#[derive(Debug, Clone)]
pub struct Symbol {
    pub target_currency: String,
    pub base_currency: String,
}

impl Symbol {
    
    // creates a new symbol
    pub fn new(target_currency: String, base_currency: String) -> Symbol {
        Symbol {
            target_currency,
            base_currency,
        }
    }
}

// represents a list of support exchanges
#[derive(Debug, Clone)]
pub enum SupportedExchange {
    Binance,
    Bitfinex,
    Bitmex,
    Bittrex,
    Bitstamp,
    Coinbase,
    Deribit,
    Ftx,
    Gemini,
    Hitbtc,
    Huobi,
    Kraken,
    KuCoin,
    Okex,
    Poloniex,
    Upbit,
}

// represents an exchange
#[derive(Debug, Clone)]
pub struct Exchange {
    pub name: String,
    pub symbols: Vec<Symbol>,
}

impl Exchange {

    // creates a new exchange
    pub fn new(name: String, symbols: Vec<Symbol>) -> Exchange {
        Exchange {
            name,
            symbols,
        }
    }
}