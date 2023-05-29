use super::bar::Bar;


// represents a query result from the database
// stores an id to use for the next query
// stores a vector of bar sets of many different symbols and exchanges
#[derive(Clone, Debug)]
pub struct QueryResult {
    pub query_id: String,
    pub status: String,
    bars: Vec<Bar>,
}

impl QueryResult {
    pub fn new(query_id: String, status: String) -> QueryResult {
        QueryResult {
            query_id,
            status,
            bars: vec![],
        }
    }

    pub fn new_with(query_id: String, status: String, bars: Vec<Bar>) -> QueryResult {
        QueryResult {
            query_id,
            status,
            bars,
        }
    }
}

impl Iterator for QueryResult {
    type Item = Bar;

    // return the next bar
    fn next(&mut self) -> Option<Self::Item> {
        // if there are no more bars, return None
        if self.bars.len() == 0 {
            return None;
        }

        // return the next bar
        Some(self.bars.remove(0))
    }
}