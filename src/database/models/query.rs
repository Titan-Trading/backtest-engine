use super::{exchange::Exchange, symbol::Symbol};


#[derive(Clone)]
pub struct Query {
    pub client_id: u64,
    pub symbols: Vec<(Exchange, Symbol)>,
    pub intervals: Vec<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub limit: i32
}

impl Query {
    pub fn new(client_id: u64) -> Self {
        Self {
            client_id,
            symbols: Vec::new(),
            intervals: Vec::new(),
            start_timestamp: None,
            end_timestamp: None,
            limit: 1000
        }
    }
}