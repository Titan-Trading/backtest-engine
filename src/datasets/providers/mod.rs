pub mod yahoo_finance;
pub mod google_finance;
pub mod crypto_compare;
pub mod crypto_data_download;

use std::io::Error;
use crate::database::models::candlestick::Candlestick;

// stores the data source information (returned from the search command)
pub struct DataSource {
    pub id: i32,
    pub data_provider: String,
    pub exchange: String,
    pub symbol: String,
    pub start_timestamp: i64,
    pub end_timestamp: Option<i64>,
}

// trait for data providers
pub trait DataProvideable {
    fn name(&self) -> String;
    fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<DataSource>, Error>;
    fn download(&self, data_source_id: i32) -> Result<Vec<Candlestick>, Error>;
}