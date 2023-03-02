use std::collections::HashMap;

use super::{database::{Bar, ConsolidatedBarSet}, query::QueryResult};


// to allow us to cache the data in memory
pub struct InMemoryCache {
    pub data: HashMap<String, Vec<ConsolidatedBarSet>>,
}

impl InMemoryCache {
    
    // create a new cache
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // create a new cache with data
    pub fn new_from(data: HashMap<String, Vec<ConsolidatedBarSet>>) -> Self {
        Self {
            data,
        }
    }

    // get data from the cache
    pub fn get(&mut self, key: &String) -> Option<QueryResult> {

        if let Some(data) = self.data.get(key) {
            println!("cache hit");
            return Some(QueryResult::new_with(data.clone()));
        } 
            
        println!("cache miss");
        return None;
    }

    // set data in the cache
    pub fn set(&mut self, key: String, value: Vec<ConsolidatedBarSet>) {
        self.data.insert(key, value);
    }

    // append data to an item in the cache using key
    pub fn append(&mut self, key: String, value: Vec<ConsolidatedBarSet>) {
        if let Some(data) = self.data.get_mut(&key) {
            data.extend(value);
        }
    }
}