use std::{io::Error, collections::HashMap};
use super::{database::Database, query::{Query, QueryResult}};

// represents a database client
pub struct Client {
    pub id: u64,
    pub database: Database,
    pub is_connected: bool,
}

impl Client {
    
    // creates a new database client
    pub fn new(id: u64, database: Database) -> Client {
        Client {
            id,
            database,
            is_connected: false,
        }
    }

    // connects the client to the database
    pub fn connect(&mut self) {
        self.is_connected = true;
        self.database.connect_client(self.id);
    }

    // disconnects the client from the database
    pub fn disconnect(&mut self) {
        self.is_connected = false;
        self.database.disconnect_client(self.id);
    }

    // gets historical data from the database using a query
    pub fn get_historical_data(&mut self, query: Query, parameters: HashMap<String, String>) -> Result<QueryResult, String> {
        Ok(self.database.query(self.id, query, parameters).unwrap())
    }

    // gets live data from the database using a query
    pub fn stream<F>(&self, query: Query, on_update: F) -> Result<bool, String>
    where
        F: Fn(Query) -> Result<QueryResult, Error>, {
        self.database.stream(self.id, query, on_update)
    }
}