use std::{io::stdin, time::Instant};
use crate::database::{database::Database, models::{symbol::Symbol, exchange::Exchange, query::Query}};
mod database;


// since we now have a filesystem with our own database file format
// we can create a custom database for accessing our market data from our filesystem
// our queries are all exclusive
// - the query will fail if there's no market data for a given exchange/symbol combination
// -- we want to take care not to load any data that cannot be sync'd as this whole database if for backtesting strategies
// -- we would rather the system fail than have a strategy run with bad data
// -- we can always add more data later
// - our queries consist of a limit of bars, start_time, end_time, intervals, symbols, and exchanges
// -- this is a very powerful query system because it allows us to get exactly what we need in the format we need it
// - we can perform optimizations behind the scenes to speed up the system
// - we can use multi-threading and specific seek/read optimizations to speed up the system
// we can cache our data in memory for faster access
// we can also support a live data feed from the database


fn main() {

    // create a new database instance
    let database = Database::new();

    // create a new client
    let client_id = 1;

    // variables for performance testing
    let mut record_count = 0;
    let start = Instant::now();

    // generate custom query to database
    let mut query = Query::new_with(client_id, database)
        // defaults to 1000
        .with_limit(2000)

        // defaults to full datasets
        .with_start_time(1577836860)
        .with_end_time(1609459140)

        // limited to 5 intervals for now
        .with_intervals(Vec::from([
            "5m".to_string(),
            "15m".to_string(),
            "1h".to_string(),
            "4h".to_string(),
            "1d".to_string()
        ]))

        // limited to 5 symbols for now
        .with_symbols(Vec::from([
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("BTC".to_string(), "USDT".to_string())), 
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("ADA".to_string(), "USDT".to_string())),
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("DASH".to_string(), "USDT".to_string()))
        ]))

        // initialize the query (generate the id and start the tasks)
        .start();

    println!("starting query: {}", query.id.clone().unwrap());

    // get historical data from database
    // - when at least one barset is stored in the cache, the query will return
    // - next() can be called multiple times to get the next barset
    let mut last_timestamp = 0;
    while let Ok(result) = query.next() {
        if result.bars.len() == 0 {
            continue;
        }

        // track total records
        record_count += result.bars.len();

        // loop through the bars
        for bar in result.bars {
            if bar.timestamp < last_timestamp {
                println!("main: timestamp error: {} < {}", bar.timestamp, last_timestamp);
                return;
            }

            last_timestamp = bar.timestamp;
        }

        if result.status == "complete" {
            break;
        }
    }

    println!("main: query id: {}", query.id.clone().unwrap());
    println!("main: symbols: {}", query.symbols.clone().len());
    println!("main: total results: {}", record_count);
    println!("main: query completed in {:?}s", (start.elapsed().as_millis() as f64 / 1000.0));

    // get live data from database

    // wait to exit
    println!("Press any key to continue");
    stdin().read_line(&mut String::new()).unwrap();
}