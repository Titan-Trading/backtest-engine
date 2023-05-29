use std::collections::HashMap;
use std::io::Error;

use super::providers::DataProvideable;
use super::providers::crypto_compare::CryptoCompare;
use super::providers::crypto_data_download::CryptoDataDownload;
use super::providers::google_finance::GoogleFinance;
use super::providers::yahoo_finance::YahooFinance;


pub struct DataProviderManager {
    pub data_providers: Vec<Box<dyn DataProvideable>>,
}

impl DataProviderManager {
    pub fn new() -> Self {

        // load data providers from config file
        let mut data_providers: Vec<Box<dyn DataProvideable>> = vec![];
        data_providers.push(Box::new(YahooFinance::new("".to_string(), "".to_string())));
        data_providers.push(Box::new(GoogleFinance::new()));
        data_providers.push(Box::new(CryptoCompare::new("".to_string())));
        data_providers.push(Box::new(CryptoDataDownload::new()));

        Self {
            data_providers,
        }
    }

    // search each data provider for the exchange and symbol
    pub fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<String>, Error> {

        // loop through each data provider
        for provider in &self.data_providers {
            // make http request to data provider
            println!("Searching for {} - {} on {}", symbol, exchange, provider.name());

            provider.search(exchange.clone(), symbol.clone(), start_timestamp, end_timestamp);
        }

        Ok(vec![])
    }

    // download data from data provider
    pub fn download(&self, provider_name: String, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<String>, Error> {

        // loop through each data provider
        for provider in &self.data_providers {
            // if name == &provider_name {
                // make http request to data provider
                println!("Downloading {} - {} from {}", symbol, exchange, provider.name());

                provider.download(exchange.clone(), symbol.clone(), start_timestamp, end_timestamp);
            // }
        }

        Ok(vec![])
    }
}