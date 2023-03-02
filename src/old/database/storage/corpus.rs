use std::collections::HashMap;
use std::io::Error;
use crate::database::database::{Bar, BarSet, Exchange, Symbol};
use crate::database::query::Query;


// represents a corpus engine (read/write data from/to the filesystem)
pub trait CorpusEngine {
    // read list of files from the filesystem returns vector of barsets (whole files)
    fn read_files(&mut self, files: Vec<String>, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error>;

    // read list of files from the filesystem returns a vector of barsets (chunks of files)
    fn read_chunk(&mut self, files: Vec<String>, parameters: &HashMap<String, String>) -> Result<HashMap<String, Vec<Bar>>, Error>;

    // write list of files to the filesystem, returns vector of bools to show if write was successful
    fn write_files(&mut self, files: Vec<String>) -> Result<Vec<bool>, Error>;
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

// represents an indice of all the files in the corpus
pub struct CorpusIndex {
    // name of the corpus index file
    pub filename: String,

    pub root_dir: String,

    // map of symbol and exchange combos to corpus files
    pub corpus_map: HashMap<String, Corpus>,
}
impl CorpusIndex {

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
        for (exchange, symbol) in search_query.symbols.unwrap() {

            let symbol_name = format!("{}{}", symbol.target_currency, symbol.base_currency);
            let data_name = format!("{}_{}", exchange.name, symbol_name);
            // let data_filename = format!("{}.json", data_name);

            let corpus = Corpus::new(0, exchange.name.clone(), symbol_name, 0, 0, data_name.clone(), 0);
            
            corpus_map.insert(data_name, corpus);
        }
        // for exchange in search_query.exchanges.unwrap() {
        //     let exchange = exchange.to_string().clone();
        //     let symbols = search_query.symbols.clone().unwrap();
            
        // }

        CorpusIndex::new_from(filename, root_dir, corpus_map)
    }
}