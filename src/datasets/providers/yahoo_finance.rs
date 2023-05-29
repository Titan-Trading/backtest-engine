use std::io::Error;
use yahoo_finance::{history, Interval};
use crate::{utils::rest::Client, database::models::candlestick::Candlestick};
use super::DataProvideable;

pub struct YahooFinanceResponse {
    pub data: Vec<Candlestick>,
}

pub struct YahooFinance {
    pub client: Client,
    pub api_key: String,
    pub api_secret: String,
}

impl YahooFinance {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_secret,
        }
    }
}

impl DataProvideable for YahooFinance {

    // returns the name of the provider
    fn name(&self) -> String {
        "YahooFinance".to_string()
    }

    // searches to see if there's data for the exchange and symbol on yahoo finance
    fn search(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<bool, Error> {
        // let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1m&range=1000&includePrePost=false&events=div%7Csplit%7Cearn&lang=en-US&region=US&corsDomain=finance.yahoo.com", symbol);
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        // let res: YahooFinanceResponse = serde_json::from_str(&body)?;

        // convert i64 unix utc timestamp to chrono::DateTime
        // let start_datetime_naive = chrono::NaiveDateTime::from_timestamp_opt(start_timestamp, 0);
        // let start_datetime = chrono::DateTime::from_utc(start_datetime_naive.unwrap(), chrono::Utc);

        // let res = null;
        // if let Some(timestamp) = end_timestamp {
        //     let end_datetime_naive = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
        //     let end_datetime = chrono::DateTime::from_utc(end_datetime_naive.unwrap(), chrono::Utc);
        //     res = history::retrieve_range(&symbol, start_datetime, Some(end_datetime));
        // }
        // else {
        //     res = history::retrieve_range(&symbol, start_datetime, None);
        // }

        Ok(false)
    }

    // gets data from yahoo finance api
    fn download(&self, exchange: String, symbol: String, start_timestamp: i64, end_timestamp: Option<i64>) -> Result<Vec<Candlestick>, Error> {
        // let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1m&range=1000&includePrePost=false&events=div%7Csplit%7Cearn&lang=en-US&region=US&corsDomain=finance.yahoo.com", symbol);
        // let mut response = self.client.get(&url).unwrap();
        // let body = response.text();
        // let data: YahooFinanceResponse = serde_json::from_str(&body)?;
        let mut candlesticks: Vec<Candlestick> = Vec::new();

        // for candlestick in data.chart.result[0].indicators.quote[0] {
        //     candlesticks.push(Candlestick {
        //         open: candlestick,
        //         high: candlestick,
        //         low: candlestick,
        //         close: candlestick,
        //         volume: candlestick,
        //         timestamp: candlestick,
        //     });
        // }

        Ok(candlesticks)
    }
}