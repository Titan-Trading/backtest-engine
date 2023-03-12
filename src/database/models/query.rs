use std::{io::{Error, ErrorKind}, collections::HashMap};
use crate::database::database::Database;
use super::{exchange::Exchange, symbol::Symbol, query_result::QueryResult};


// represents a query to the database
#[derive(Clone)]
pub struct Query {
    pub id: Option<String>,
    pub client_id: Option<u64>,
    pub database: Database,
    pub symbols: Vec<(Exchange, Symbol)>,
    pub intervals: Vec<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub limit: i32
}

impl Query {
    // creates a new query using database instance
    pub fn new(database: Database) -> Self {
        Self {
            id: None,
            client_id: None,
            database,
            symbols: Vec::new(),
            intervals: Vec::new(),
            start_timestamp: None,
            end_timestamp: None,
            limit: 1000
        }
    }

    // creates a new query using database instance and client id
    pub fn new_with(client_id: u64, database: Database) -> Self {
        Self {
            id: None,
            client_id: Some(client_id),
            database,
            symbols: Vec::new(),
            intervals: Vec::new(),
            start_timestamp: None,
            end_timestamp: None,
            limit: 1000
        }
    }

    // sets the limit of the query (default is 1000, max is 10000)
    pub fn with_limit(mut self, limit: i32) -> Self {

        // limit to 10000 for performance
        if limit > 10000 {
            panic!("Limit to 10000 for performance");
        }

        self.limit = limit;
        self
    }

    // sets the start time of the query
    pub fn with_start_time(mut self, start_timestamp: i64) -> Self {
        self.start_timestamp = Some(start_timestamp);
        self
    }

    // sets the end time of the query
    pub fn with_end_time(mut self, end_timestamp: i64) -> Self {
        self.end_timestamp = Some(end_timestamp);
        self
    }

    // sets the symbols of the query (limit to 5 for performance)
    pub fn with_symbols(mut self, symbols: Vec<(Exchange, Symbol)>) -> Self {

        // limit to 5 symbols for performance
        if symbols.len() > 5 {
            panic!("Limit to 5 symbols for performance");
        }

        self.symbols = symbols;
        self
    }

    // sets the intervals of the query (limit to 5 for performance)
    pub fn with_intervals(mut self, intervals: Vec<String>) -> Self {

        // limit to 5 intervals for performance
        if intervals.len() > 5 {
            panic!("Limit to 5 intervals for performance");
        }

        self.intervals = intervals;
        self
    }

    // starts the query with the database instance
    pub fn start(self) -> Self {

        // get results from the database instance
        let query = self.clone();
        let mut database = self.database.clone();
        let client_id = self.client_id;
        let results = database.query(client_id.unwrap(), query).unwrap();

        Self {
            id: Some(results.id), // set the query id
            client_id: client_id,
            database: database,
            symbols: self.symbols,
            intervals: self.intervals,
            start_timestamp: self.start_timestamp,
            end_timestamp: self.end_timestamp,
            limit: self.limit
        }
    }

    // use database instance to get results from the query that was started
    pub fn next(&mut self) -> Result<QueryResult, Error> {

        // a query id is required to get the next results
        if self.id.is_none() {
            return Err(Error::new(ErrorKind::Other, "Query id is required to get next results"));
        }
        let query_id = self.id.clone().unwrap();

        // get results from the database instance
        let parameters: HashMap<String, String> = HashMap::new();
        let results = self.database.query_chunk(query_id, parameters);
        if let Err(error) = results {
            return Err(error);
        }

        Ok(results.unwrap())
    }
}