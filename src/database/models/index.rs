use std::collections::HashMap;
use super::query::Query;


// represents an indice of all the files in the corpus
#[derive(Debug, Clone)]
pub struct DatabaseIndex {
    // name of the corpus index file
    pub filename: String,

    pub root_dir: String,

    // map of symbol and exchange combos to corpus files
    pub corpus_map: HashMap<String, Corpus>,
}
impl DatabaseIndex {

    // create new empty index
    pub fn new(filename: String, root_dir: String) -> Self {
        Self {
            filename,
            root_dir,
            corpus_map: HashMap::new(),
        }
    }

    // create new index from existing map
    pub fn new_from(filename: String, root_dir: String, corpus_map: HashMap<String, Corpus>) -> Self {
        Self {
            filename,
            root_dir,
            corpus_map,
        }
    }

    // convert a database query into a list of files that need to be read
    pub fn from_query(filename: String, root_dir: String, query: Query) -> Self {

        let search_query = query.clone();
        let mut corpus_map = HashMap::new();

        // get every file for every symbol and exchange combo
        for (exchange, symbol) in search_query.symbols {

            let symbol_name = format!("{}{}", symbol.target_currency, symbol.base_currency);
            let data_name = format!("{}_{}", exchange.name, symbol_name);
            // let data_filename = format!("{}.json", data_name);

            let corpus = Corpus::new(0, exchange.name.clone(), symbol_name, 0, 0, data_name.clone(), 0);
            
            corpus_map.insert(data_name, corpus);
        }

        DatabaseIndex::new_from(filename, root_dir, corpus_map)
    }
}

// corpus index
// stores a list of files and their corresponding bar information
// provides a query way to convert a query into a list of files that need to be read

// represents a single file within the corpus (could be more than one file for efficiency later)
#[derive(Debug, Clone)]
pub struct Corpus {
    pub dataset_id: u8,
    pub last_updated: i32,
    pub exchange: String,
    pub symbol: String,
    pub start_timestamp: i32,
    pub end_timestamp: i32,
    pub filename: String,
}
impl Corpus {
    
    // create new corpus file
    pub fn new(dataset_id: u8, exchange: String, symbol: String, start_timestamp: i32, end_timestamp: i32, filename: String, last_updated: i32) -> Self {
        Self {
            last_updated,
            dataset_id,
            exchange,
            symbol,
            start_timestamp,
            end_timestamp,
            filename,
        }
    }
}