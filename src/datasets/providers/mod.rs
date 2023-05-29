pub mod yahoo_finance;
pub mod google_finance;
pub mod crypto_compare;
pub mod crypto_data_download;

use std::io::Error;

use crate::database::models::candlestick::Candlestick;


pub trait DataProvideable {
    fn name(&self) -> String;
    fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<bool, Error>;
    fn download(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<Candlestick>, Error>;
}