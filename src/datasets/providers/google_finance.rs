use std::io::Error;
use crate::{database::models::candlestick::Candlestick, utils::rest::Client};

use super::DataProvideable;


pub struct GoogleFinance {
    client: Client,
}

impl GoogleFinance {
    pub fn new() -> GoogleFinance {
        GoogleFinance {
            client: Client::new()
        }
    }
}

impl DataProvideable for GoogleFinance {

    // returns name of provider
    fn name(&self) -> String {
        "GoogleFinance".to_string()
    }

    // searches google finance api for exchange and symbol
    fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<bool, Error> {
        // let url = format!("https://www.google.com/finance/historical?q={}.{}&startdate=Jan+1%2C+2010&enddate=Dec+31%2C+2018&output=csv", symbol, exchange);
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        // let mut candlesticks: Vec<Candlestick> = Vec::new();

        Ok(false)
    }

    // gets data from google finance api
    fn download(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<Candlestick>, Error> {
        // let url = format!("https://www.google.com/finance/historical?q={}.{}&startdate=Jan+1%2C+2010&enddate=Dec+31%2C+2018&output=csv", symbol, exchange);
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        let mut candlesticks: Vec<Candlestick> = Vec::new();

        // parse csv
        // let mut rdr = csv::Reader::from_reader(body.as_bytes());
        // for result in rdr.records() {
        //     let record = result?;
        //     let timestamp = record[0];
        //     let open = record[1];
        //     let high = record[2];
        //     let low = record[3];
        //     let close = record[4];
        //     let volume = record[5];

        //     candlesticks.push(Candlestick {
        //         open: open.parse::<f64>()?,
        //         high: high.parse::<f64>()?,
        //         low: low.parse::<f64>()?,
        //         close: close.parse::<f64>()?,
        //         volume: volume.parse::<f64>()?,
        //         timestamp: timestamp.parse::<i64>()?,
        //     });
        // }

        Ok(candlesticks)
    }
}