
// since we now have a filesystem
// with our own database file format
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

fn main() {

    // // generate custom query to database
    // let query = Query::new()
    //     .with_limit(100)
    //     .with_start_time(1598918400000)
    //     .with_end_time(1599004800000)
    //     .with_intervals(["5m", "15m", "1h", "4h", "1d"])
    //     .with_symbols(["BTC-USDT", "ETH-USDT", "LTC-USDT", "XRP-USDT"])
    //     .with_exchanges([Exchange::Binance, Exchange::Bitfinex, Exchange::Bitstamp, Exchange::Bittrex, Exchange::Coinbase, Exchange::Kraken, Exchange::Poloniex]);

    // // create a new database client
    // let mut client = Client::new();

    // // get historical data from database using custom query
    // let result = client.get_historical_data(query).unwrap();
}