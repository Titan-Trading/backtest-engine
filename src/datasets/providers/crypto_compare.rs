use std::io::Error;

use crate::{utils::rest::Client, database::models::candlestick::Candlestick};

use super::DataProvideable;


pub struct CryptoCompareResponse {
    data: Vec<Candlestick>,
}

pub struct CryptoCompare {
    pub client: Client,
    pub api_key: String,
}

impl CryptoCompare {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

impl DataProvideable for CryptoCompare {

    // returns the name of the provider
    fn name(&self) -> String {
        "CryptoCompare".to_string()
    }

    // searches for data on cryptocompare.com
    fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<bool, Error> {
        // let url = format!("https://min-api.cryptocompare.com/data/v2/histominute?e={}&fsym={}&tsym=USD&limit=1000&api_key={}", exchange, symbol, self.api_key);
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        // let data: CryptoCompareResponse = serde_json::from_str(&body)?;

        // if data.Data.Data.len() == 0 {
        //     return Err("No data found".into());
        // }

        // Ok(vec![data.Data.Data[0].time.to_string()])

        Ok(true)
    }

    // gets data from cryptocompare.com
    fn download(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<Candlestick>, Error> {
        // let url = format!("https://min-api.cryptocompare.com/data/v2/histominute?e={}&fsym={}&tsym=USD&limit=1000&toTs={}&api_key={}", exchange, symbol, end_timestamp, self.api_key);
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        // let data: CryptoCompareResponse = serde_json::from_str(&body)?;
        let mut candlesticks: Vec<Candlestick> = Vec::new();

        // for candlestick in data.Data.Data {
        //     candlesticks.push(Candlestick {
        //         open: candlestick.open,
        //         high: candlestick.high,
        //         low: candlestick.low,
        //         close: candlestick.close,
        //         volume: candlestick.volumefrom,
        //         timestamp: candlestick.time,
        //     });
        // }

        Ok(candlesticks)
    }
}