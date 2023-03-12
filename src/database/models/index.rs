use std::{collections::HashMap, io::{ErrorKind, Error, Write}, fs::File};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use super::query::Query;


// represents the schema of the index file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseIndexFile {
    last_updated: i64,
    last_file_id: i32,
    file_map: HashMap<String, Corpus>,
}


// represents an indice of all the files in the corpus
#[derive(Debug, Clone)]
pub struct DatabaseIndex {
    // name of the corpus index file
    pub filename: String,

    // root directory for the datasets
    pub root_dir: String,

    // last file id
    pub last_file_id: i32,

    // map of symbol and exchange combos to corpus files
    pub corpus_map: HashMap<String, Corpus>,
}
impl DatabaseIndex {

    // create index and read from json file
    pub fn new(filename: String, root_dir: String) -> Self {

        // read index json file
        let mut file = File::open(filename.clone()).unwrap();

        // use serde to deserialize json into a hashmap
        let dataset_index: DatabaseIndexFile = serde_json::from_reader(&mut file).unwrap();

        // create new corpus map 
        let corpus_map = dataset_index.file_map;

        Self {
            filename,
            root_dir,
            last_file_id: dataset_index.last_file_id,
            corpus_map,
        }
    }

    // create new index from existing map
    pub fn new_from(filename: String, root_dir: String, last_file_id: i32, corpus_map: HashMap<String, Corpus>) -> Self {
        Self {
            filename,
            root_dir,
            last_file_id,
            corpus_map,
        }
    }

    // convert a database query into a list of files that need to be read
    pub fn from_query(&self, query: Query) -> Self {
        let search_query = query.clone();
        let mut corpus_map = HashMap::new();

        // get every file for every symbol and exchange combo
        for (exchange, symbol) in search_query.symbols {
            let symbol_name = format!("{}{}", symbol.target_currency, symbol.base_currency);
            let exchange_name = exchange.name.clone();

            let result = self.corpus_map.get(&format!("{}_{}", exchange_name, symbol_name));
            if let None = result {
                continue;
            }
            let corpus = result.unwrap().clone();

            let symbol_name = format!("{}{}", symbol.target_currency, symbol.base_currency);
            let data_name = format!("{}_{}", exchange.name, symbol_name);

            let corpus = Corpus::new(corpus.file_id, exchange.name.clone(), symbol_name, corpus.start_timestamp, corpus.end_timestamp, data_name.clone(), 0);
            
            corpus_map.insert(data_name, corpus);
        }

        DatabaseIndex::new_from(self.filename.clone(), self.root_dir.clone(), self.last_file_id + 1, corpus_map)
    }

    // get the file information using exchange and symbol string tuple
    pub fn get_info(&mut self, (exchange, symbol): (String, String)) -> Result<Corpus, Error> {
        if let Some(file_info) = self.corpus_map.get(&format!("{}_{}", exchange, symbol)) {
            return Ok(file_info.clone());
        }
        
        Err(Error::new(ErrorKind::NotFound, format!("Could not find file for {} {}", exchange, symbol)))
    }

    // add a new file to the index
    pub fn add_file(&mut self, file: Corpus) -> Result<bool, Error> {
        let symbol_name = format!("{}_{}", file.exchange, file.symbol);
        if let None = self.corpus_map.insert(symbol_name, file) {
            return Ok(true);
        }

        Ok(false)
    }

    // save the index to a json file
    pub fn save(&self) -> Result<bool, Error> {
        let mut file = File::create(self.filename.clone()).unwrap();

        let index_file = DatabaseIndexFile {
            last_updated: Utc::now().timestamp(),
            last_file_id: self.last_file_id,
            file_map: self.corpus_map.clone(),
        };

        let json = serde_json::to_string(&index_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        Ok(true)
    }
}

// corpus index
// stores a list of files and their corresponding bar information
// provides a query way to convert a query into a list of files that need to be read

// represents a single file within the corpus (could be more than one file for efficiency later)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corpus {
    pub file_id: i32,
    pub last_updated: i64,
    pub exchange: String,
    pub symbol: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub filename: String,
}
impl Corpus {
    
    // create new corpus file
    pub fn new(file_id: i32, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: i64, filename: String, last_updated: i64) -> Self {
        Self {
            last_updated,
            file_id,
            filename,
            exchange,
            symbol,
            start_timestamp,
            end_timestamp,
        }
    }
}