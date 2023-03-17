use std::collections::HashMap;
use std::io::Error;


pub struct DataProvider {
    pub name: String,
    pub url: String,
}

impl DataProvider {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
        }
    }
}



pub struct DataProviderManager {
    pub data_providers: HashMap<String, DataProvider>,
}

impl DataProviderManager {
    pub fn new() -> Self {
        Self {
            data_providers: HashMap::new(),
        }
    }

    pub fn search(&self, exchange: String, symbol: String) -> Result<Vec<String>, Error> {
        
        // make http request to data provider


        Ok(vec![])
    }
}