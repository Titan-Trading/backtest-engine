use std::collections::HashMap;
use std::io::Error;
use super::{engine::DatabaseEngine, models::{query::Query, query_result::QueryResult, bar::Bar}};

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

    // gets historical data from the database using a query and fill cache
    // gets first chunk and returns a query id to further chunks of data
    pub fn query(&mut self, client_id: u64, query: Query) -> Result<QueryResult, Error> {
        Ok(self.engine.start_query(client_id, query).unwrap())
    }

    // gets historical data from the database using a query id from cache
    // parameters: index - index in number of records (a multiplied value of bytes)
    pub fn query_chunk(&mut self, query_id: String, parameters: HashMap<String, String>) -> Result<QueryResult, Error> {
        Ok(self.engine.query_chunk(query_id, parameters).unwrap())
    }

    // inserts data into the database
    pub fn insert(&self, client_id: u64, data: Vec<Bar>) -> Result<bool, Error> {
        // Ok(self.engine.insert(client_id, data))
        Ok(true)
    }
}