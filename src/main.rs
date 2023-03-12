// Simple Trader Market Data Binary file format (.stmdb)
// - header (custom first 4 bytes "STMD", dataset id, start timestamp, end timestamp)
// - apply optimized compression for number data in series
// -- storing series data in text format is inefficient for both read and disk space
// - store all symbols together in one exchange dataset (omit sections/bytes of symbols you don't care about)
// -- to add new symbols to an exchange dataset it will require an automated process of converting csv data into existing stmdb data
// - chunk datasets into separate files so we can load data in parallel from multiple threads
// - separate datasets by exchange which is the name of the folder
// - each bar is a byte stream which ends in 0x1337
// - each bar goes in the order unix timestamp, open, high, low, close, volume, 0x1337
// convert csv data into stmdb files
// load stmdb files and loop through them


use std::{collections::HashMap, fs::{File, self}, io::{Read}};
use database::{database::Database, models::{query::Query, exchange::Exchange, symbol::Symbol}};

use crate::database::models::candlestick::Candlestick;
mod database;

const ROOT_DATA_DIR: &str = "./data/";

fn csv_to_stmdb(mut db: Database, exchange: String, symbol: String) {

    let csv_file = format!("{}{}{}_{}.csv", ROOT_DATA_DIR, "raw/", exchange, symbol);
    let stmdb_file = format!("{}{}_{}.stmdb", ROOT_DATA_DIR, exchange, symbol);

    println!("deleting any conflicting files...");
    if let Ok(res) = fs::remove_file(stmdb_file.clone()) {
        println!("deleted {}", stmdb_file);
    }

    println!("converting {} to {}", csv_file, stmdb_file);

    // read csv file
    let mut file = File::open(csv_file.clone()).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).expect("failed to read csv file");

    // read csv data in reverse order (bottom to top)
    let mut start_timestamp = 0;
    let mut line_count = 1;
    let mut data: Vec<Candlestick> = vec![];
    for line in text.lines().rev() {
        let fields = line.trim().split(",").collect::<Vec<&str>>();
        
        // skip empty lines
        if fields.len() == 0 {
            continue;
        }

        // skip header
        let header_check = fields[0].parse::<i64>();
        if let Err(err) = header_check {
            continue;
        }
        else if header_check.unwrap() == 0 {
            continue;
        }

        // create candlestick from csv data
        let timestamp = fields[0].parse::<i64>().unwrap();
        let open = fields[3].parse::<f64>().unwrap();
        let high = fields[4].parse::<f64>().unwrap();
        let low = fields[5].parse::<f64>().unwrap();
        let close = fields[6].parse::<f64>().unwrap();
        let volume = fields[7].parse::<f64>().unwrap();

        // println!("{} {} {} {} {} {}", timestamp, open, high, low, close, volume);

        let record = Candlestick::new_with(timestamp, open, high, low, close, volume);
        data.push(record);

        // insert chunk into database
        if line_count % 1000 == 0 {
            let data_to_add = data.clone();
            db.insert(1, exchange.clone(), symbol.clone(), data_to_add).unwrap();
            println!("inserted {} records", data.len());

            // reset chunk
            data = vec![];
        }

        line_count += 1;
    }

    // insert remaining chunk into database
    let data_to_add = data.clone();
    db.insert(1, exchange, symbol, data_to_add).unwrap();
    
    println!("inserted {} records", data.clone().len());
    println!("total lines: {}", line_count);
}

fn load_stmdb(mut db: Database) {
    let database = db.clone();

    let query_builder = Query::new_with(1, database)
        // defaults to 1000
        .with_limit(1000)

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
        .with_symbols([
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("DASH".to_string(), "USDT".to_string())),
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("BTC".to_string(), "USDT".to_string())),
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("ADA".to_string(), "USDT".to_string())),
        ].to_vec())
        .start();

    let query = db.query(1, query_builder).unwrap();

    while let Ok(data) = db.query_chunk(query.id.clone(), HashMap::new()) {
        // stop when status changes
        if data.status == "complete" {
            break;
        }

        // skip empty data
        if data.bars.len() == 0 {
            continue;
        }

        println!("status: {}", data.status);
        println!("got {} records", data.bars.len());
    }

    /*let csv_file = format!("{}_{}.csv", exchange, symbol);
    let stmdb_file = format!("{}_{}.stmdb", exchange, symbol);

    println!("converting {} to {}", stmdb_file, csv_file);

    // stmdb file can have index and limit parameters for reading
    let mut parameters: HashMap<String, String> = HashMap::new();
    let records = match Reader::new_with(FileType::Stmdb).read_file(stmdb_file.clone(), parameters) {
        Ok(ReaderResult::ByteRecords(records)) => panic!("expected stmdb records, got byte records"),
        Ok(ReaderResult::String(records)) => panic!("expected stmdb records, got string records"),
        Ok(ReaderResult::STMDBIndex(records)) => panic!("expected stmdb records, got stmdb index"),
        Ok(ReaderResult::STMDBRecords(records)) => records,
        Err(e) => panic!("failed to read stmdb file: {}", e),
    };

    // convert byte records
    let byte_records = records.iter().map(|record| {
        let mut text_record = StringRecord::new();
        text_record.push_field(record.timestamp.to_string().as_str());
        text_record.push_field(record.open.to_string().as_str());
        text_record.push_field(record.high.to_string().as_str());
        text_record.push_field(record.low.to_string().as_str());
        text_record.push_field(record.close.to_string().as_str());
        text_record.push_field(record.volume.to_string().as_str());

        let byte_record = ByteRecord::from(text_record);
        byte_record
    }).collect();

    // csv file can have append parameters for writing
    let parameters: HashMap<String, String> = HashMap::new();
    let written = Writer::new(FileType::Csv).write_file(csv_file.clone(), WriterInput::ByteRecords(byte_records), parameters).expect("failed to write csv file");
    if written {
        println!("successfully wrote csv file");
    } else {
        println!("failed to write csv file");
    }*/
}

fn main() {

    let DB = Database::new();

    // define input files
    let mut datasets = Vec::new();
    datasets.push(("KuCoin", ("DASH", "USDT"), 1, 1577836860, 1609459140));
    datasets.push(("KuCoin", ("BTC", "USDT"), 2, 1577836860, 1609459140));
    datasets.push(("KuCoin", ("ADA", "USDT"), 3, 1577836860, 1609459140));

    println!("{} datasets found", datasets.len());

    // loop through datasets
    for (index, dataset) in datasets.iter().enumerate() {
        let exchange = dataset.0;
        let target_currency = dataset.1.0;
        let base_currency = dataset.1.1;
        let dataset_id = dataset.2;
        let start_timestamp = dataset.3;
        let end_timestamp = dataset.4;

        // convert csv data into stmdb files
        csv_to_stmdb(DB.clone(), exchange.to_string(), format!("{}{}", target_currency, base_currency));
    }

    // load stmdb files and loop through them
    load_stmdb(DB.clone());

}