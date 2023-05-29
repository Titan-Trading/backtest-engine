use std::collections::HashMap;
use std::io::Error;
use super::{engine::DatabaseEngine, models::{query_result::QueryResult, candlestick::Candlestick, index::DatabaseIndex, query::Query}};

// Database struct. It is used to represent the database system and all of its clients.
// we support the stmdb file format
// we support the csv file format
// we support the ability to get live data

#[derive(Clone)]
pub struct Database {
    engine: DatabaseEngine,
    clients: Vec<u64>,
}

impl Database {

    // creates a new database
    pub fn new() -> Database {
        Database {
            engine: DatabaseEngine::new(),
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

    // get the database index
    pub fn get_index(&mut self) -> Result<DatabaseIndex, Error> {
        Ok(self.engine.get_index())
    }

    // gets historical data from the database using a query and fill cache
    // returns a query iterator
    pub fn start_query(&mut self, client_id: u64, query: Query) -> Result<QueryResult, Error> {
        Ok(self.engine.start_query(client_id, query).unwrap())
    }

    // get a chunk of data from a query using parameters ex: page
    pub fn query_chunk(&mut self, query_id: String, parameters: HashMap<String, String>) -> Result<QueryResult, Error> {
        Ok(self.engine.query_chunk(query_id, parameters).unwrap())
    }

    // stops a query
    pub fn stop_query(&mut self, query_id: String) -> Result<bool, Error> {
        Ok(self.engine.stop_query(query_id).unwrap())
    }

    // inserts data into the database
    pub fn insert(&mut self, client_id: u64,exchange: String, symbol: String, data: Vec<Candlestick>) -> Result<bool, Error> {
        Ok(self.engine.insert(client_id, exchange, symbol, data).unwrap())
    }
}