use std::io::Error;
use chrono::Utc;
use crate::{database::models::candlestick::Candlestick, utils::rest::Client};
use super::DataProvideable;


pub struct CryptoDataDownload {
    pub client: Client,
}

impl CryptoDataDownload {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl DataProvideable for CryptoDataDownload {

    // name of provider
    fn name(&self) -> String {
        "CryptoDataDownload".to_string()
    }

    // search for exchange and symbol
    fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<bool, Error> {
        // let url = format!("https://www.cryptodatadownload.com/cdd/{}_{}_{}_minute.csv", exchange, symbol, Utc.timestamp(start_timestamp, 0).year());
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        // let mut candlesticks: Vec<Candlestick> = Vec::new();

        // for line in body.lines() {
        //     let mut candlestick = Candlestick::default();
        //     let mut i = 0;

        //     for value in line.split(",") {
        //         match i {
        //             0 => candlestick.timestamp = Utc.datetime_from_str(value, "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
        //             1 => candlestick.open = value.parse::<f64>().unwrap(),
        //             2 => candlestick.high = value.parse::<f64>().unwrap(),
        //             3 => candlestick.low = value.parse::<f64>().unwrap(),
        //             4 => candlestick.close = value.parse::<f64>().unwrap(),
        //             5 => candlestick.volume = value.parse::<f64>().unwrap(),
        //             _ => (),
        //         }
        //         i += 1;
        //     }

        //     candlesticks.push(candlestick);
        // }

        // println!("{:?}", candlesticks);

        Ok(true)
    }

    // gets data from cryptodatadownload api
    fn download(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<Candlestick>, Error> {
        // get all the years that span the start_timestamp and end_timestamp
        // let mut years: Vec<i32> = Vec::new();
        // let mut year = start_timestamp;
        // while year <= end_timestamp {
        //     let year = Utc.timestamp(year, 0).year();
        //     if !years.contains(&year) {
        //         years.push(year);
        //     }
        //     year += 31536000;
        // }

        // get all the data for each year
        let mut candlesticks: Vec<Candlestick> = Vec::new();
        // for year in years {
        //     let url = format!("https://www.cryptodatadownload.com/cdd/{}_{}_{}_minute.csv", exchange, symbol, year);
        //     let mut response = self.client.get(&url).unwrap();
        //     let body = response.text();

        //     for line in body.lines() {
        //         let mut candlestick = Candlestick::default();
        //         let mut i = 0;

        //         for value in line.split(",") {
        //             match i {
        //                 0 => candlestick.timestamp = value.parse::<i64>()?,
        //                 1 => candlestick.open = value.parse::<f64>()?,
        //                 2 => candlestick.high = value.parse::<f64>()?,
        //                 3 => candlestick.low = value.parse::<f64>()?,
        //                 4 => candlestick.close = value.parse::<f64>()?,
        //                 5 => candlestick.volume = value.parse::<f64>()?,
        //                 _ => (),
        //             }

        //             i += 1;
        //         }

        //         candlesticks.push(candlestick);
        //     }   
        // }

        Ok(candlesticks)
    }
}