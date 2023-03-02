use super::{barset::BarSet, bar::Bar};


// represents a query result from the database
// stores an id to use for the next query
// stores a vector of bar sets of many different symbols and exchanges
#[derive(Clone, Debug)]
pub struct QueryResult {
    pub id: String,
    pub status: String,
    pub bars: Vec<Bar>,
}

impl QueryResult {
    pub fn new(id: String, status: String) -> QueryResult {
        QueryResult {
            id,
            status,
            bars: vec![],
        }
    }

    pub fn new_with(id: String, status: String, bars: Vec<Bar>) -> QueryResult {
        QueryResult {
            id,
            status,
            bars,
        }
    }
}