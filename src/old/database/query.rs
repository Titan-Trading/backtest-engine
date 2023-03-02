use std::{collections::hash_map::DefaultHasher, hash::Hasher, io::Error};

use super::database::{Symbol, Bar, ConsolidatedBarSet, SupportedExchange, Exchange};


// represents a query to the database
#[derive(Debug, Clone)]
pub struct Query {
    pub limit: Option<u32>,
    pub start_timestamp: Option<u64>,
    pub end_timestamp: Option<u64>,
    pub intervals: Option<Vec<String>>,
    pub symbols: Option<Vec<(Exchange, Symbol)>>,
    pub exchanges: Option<Vec<String>>,
}
impl Query {

    // creates a new query
    pub fn new() -> Self {
        Self {
            limit: None,
            start_timestamp: None,
            end_timestamp: None,
            intervals: None,
            symbols: None,
            exchanges: None,
        }
    }

    // sets the limit of the query
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // sets the start time of the query
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_timestamp = Some(start_time);
        self
    }

    // sets the end time of the query
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_timestamp = Some(end_time);
        self
    }

    // sets the intervals of the query
    pub fn with_intervals(mut self, intervals: Vec<String>) -> Self {
        self.intervals = Some(intervals);
        self
    }

    // sets the symbols of the query
    pub fn with_symbols(mut self, symbols: Vec<(Exchange, Symbol)>) -> Self {
        self.symbols = Some(symbols);
        self
    }

    // returns true if the query is valid
    pub fn is_valid(&self) -> bool {
        true
    }

    // create a hash from the query
    pub fn to_hash(&self, client_id: u64) -> Result<String, Error> {

        // convert integer values to strings
        let limit = match self.limit {
            Some(limit) => limit.to_string(),
            None => "".to_string(),
        };
        let start_time = match self.start_timestamp {
            Some(start_time) => start_time.to_string(),
            None => "".to_string(),
        };
        let end_time = match self.end_timestamp {
            Some(end_time) => end_time.to_string(),
            None => "".to_string(),
        };

        // convert vector values to strings
        let intervals = match &self.intervals {
            Some(intervals) => {
                let mut intervals_string = "".to_string();
                for interval in intervals {
                    intervals_string.push_str(&interval);
                }
                intervals_string
            },
            None => "".to_string(),
        };
        let symbols = match &self.symbols {
            Some(items) => {
                let mut symbols_string = "".to_string();
                for (exchange, symbol) in items {
                    symbols_string.push_str(&exchange.name);
                    symbols_string.push_str(&symbol.target_currency.as_str());
                    symbols_string.push_str(&symbol.base_currency.as_str());
                }
                symbols_string
            },
            None => "".to_string(),
        };

        let exchanges = match &self.exchanges {
            Some(exchanges) => {
                let mut exchanges_string = "".to_string();
                for exchange in exchanges {
                    exchanges_string.push_str(&exchange.to_string());
                }
                exchanges_string
            },
            None => "".to_string(),
        };

        // concatenate all the strings
        let query_string = format!("{}{}{}{}{}{}", limit, start_time, end_time, intervals, symbols, exchanges);

        // create a hash from the string
        let mut hasher = DefaultHasher::new();
        hasher.write(query_string.as_bytes());
        hasher.write(client_id.to_string().as_bytes());
        let hash = hasher.finish().to_string();

        Ok(hash)
    }
}

// represents the result of a query
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub bars: Vec<ConsolidatedBarSet>,
}

impl QueryResult {
    
    // creates a new query result
    pub fn new() -> QueryResult {
        QueryResult {
            bars: Vec::new(),
        }
    }

    // create a new query result using a vector of barsets
    pub fn new_with(bars: Vec<ConsolidatedBarSet>) -> QueryResult {
        QueryResult {
            bars,
        }
    }
}