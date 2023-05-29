use super::{query::Query, exchange::Exchange, symbol::Symbol};


// represents a query to the database
#[derive(Clone)]
pub struct QueryBuilder {
    pub query: Query,
}

impl QueryBuilder {
    // creates a new query using database instance
    pub fn new(client_id: u64) -> Self {
        Self {
            query: Query::new(client_id)
        }
    }

    // sets the limit of the query (default is 1000, max is 10000)
    pub fn with_limit(mut self, limit: i32) -> Self {

        // limit to 10000 for performance
        if limit > 10000 {
            panic!("Limit to 10000 for performance");
        }

        self.query.limit = limit;
        self
    }

    // sets the start time of the query
    pub fn with_start_time(mut self, start_timestamp: i64) -> Self {
        self.query.start_timestamp = Some(start_timestamp);
        self
    }

    // sets the end time of the query
    pub fn with_end_time(mut self, end_timestamp: i64) -> Self {
        self.query.end_timestamp = Some(end_timestamp);
        self
    }

    // sets the symbols of the query (limit to 5 for performance)
    pub fn with_symbols(mut self, symbols: Vec<(Exchange, Symbol)>) -> Self {

        // limit to 5 symbols for performance
        if symbols.len() > 5 {
            panic!("Limit to 5 symbols for performance");
        }

        self.query.symbols = symbols;
        self
    }

    // sets the intervals of the query (limit to 5 for performance)
    pub fn with_intervals(mut self, intervals: Vec<String>) -> Self {
        // limit to 5 intervals for performance
        if intervals.len() > 5 {
            panic!("Limit to 5 intervals for performance");
        }

        self.query.intervals = intervals;
        self
    }

    // starts the query with the database instance
    pub fn get(self) -> Query {
        self.query
    }

    // use database instance to get results from the query that was started
    // pub fn next(&mut self) -> Result<QueryResult, Error> {

    //     // a query id is required to get the next results
    //     if self.id.is_none() {
    //         return Err(Error::new(ErrorKind::Other, "Query id is required to get next results"));
    //     }
    //     let query_id = self.id.clone().unwrap();

    //     // get results from the database instance
    //     let parameters: HashMap<String, String> = HashMap::new();
    //     let results = self.database.query_chunk(query_id, parameters);
    //     if let Err(error) = results {
    //         return Err(error);
    //     }

    //     Ok(results.unwrap())
    // }
}