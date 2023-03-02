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

use std::collections::HashMap;

use csv::{ByteRecord, StringRecord};
use filesystem::reader::Reader;
use filesystem::types::FileType;

use crate::filesystem::reader::ReaderResult;
use crate::filesystem::types::stmdb::STMDBRecord;
use crate::filesystem::writer::{Writer, WriterInput};

mod filesystem;

fn csv_to_stmdb(exchange: String, symbol: String, dataset_id: i32, start_timestamp: i32, end_timestamp: i32) {

    let csv_file = format!("{}_{}.csv", exchange, symbol);
    let stmdb_file = format!("{}_{}.stmdb", exchange, symbol);

    println!("converting {} to {}", csv_file, stmdb_file);

    // csv file can have index and limit parameters for reading
    let mut parameters = HashMap::new();
    // parameters.insert("index".to_string(), "0".to_string());
    // parameters.insert("limit".to_string(), "10000000".to_string());
    parameters.insert("reverse".to_string(), "true".to_string());

    // read csv file
    let records = match Reader::new_with(FileType::Csv).read_file(csv_file.clone(), parameters) {
        Ok(ReaderResult::ByteRecords(records)) => records,
        Ok(ReaderResult::String(records)) => panic!("expected byte records, got string records"),
        Ok(ReaderResult::STMDBRecords(records)) => panic!("expected byte records, got stmdb records"),
        Ok(ReaderResult::STMDBIndex(index)) => panic!("expected byte records, got stmdb index"),
        Err(e) => panic!("failed to read csv file: {}", e),
    };

    let mut stmdb_records: Vec<STMDBRecord> = vec![];
    for record in records {
        let stmdb_record = STMDBRecord {
            timestamp: String::from_utf8(record.get(0).ok_or("Missing timestamp field").unwrap().to_vec()).unwrap().parse::<i64>().unwrap(),
            open:   String::from_utf8(record.get(3).ok_or("Missing open field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            high:   String::from_utf8(record.get(4).ok_or("Missing high field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            low:    String::from_utf8(record.get(5).ok_or("Missing low field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            close:  String::from_utf8(record.get(6).ok_or("Missing close field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            volume: String::from_utf8(record.get(7).ok_or("Missing volume field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
        };
        stmdb_records.push(stmdb_record);
    }

    // stmdb file has to have dataset_id, start_timestamp, and end_timestamp parameters for writing (required for header)
    let mut parameters: HashMap<String, String> = HashMap::new();
    parameters.insert("dataset_id".to_string(), dataset_id.to_string());
    parameters.insert("start_timestamp".to_string(), start_timestamp.to_string());
    parameters.insert("end_timestamp".to_string(), end_timestamp.to_string());

    let written = Writer::new(FileType::Stmdb).write_file(stmdb_file.clone(), WriterInput::STMDBRecords(stmdb_records), parameters).expect("failed to write stmdb file");
    if written {
        println!("successfully wrote stmdb file");
    } else {
        println!("failed to write stmdb file");
    }
}

fn load_stmdb(exchange: String, symbol: String) {

    let csv_file = format!("TEST_{}_{}.csv", exchange, symbol);
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
    }
}

fn main() {

    // define input files
    let mut datasets = Vec::new();
    datasets.push(("KuCoin", "DASHUSDT", 1, 1577836860, 1609459140));
    datasets.push(("KuCoin", "BTCUSDT", 2, 1577836860, 1609459140));
    datasets.push(("KuCoin", "ADAUSDT", 3, 1577836860, 1609459140));

    println!("{} datasets found", datasets.len());

    // loop through datasets
    for dataset in datasets {
        let exchange = dataset.0;
        let symbol = dataset.1;
        let dataset_id = dataset.2;
        let start_timestamp = dataset.3;
        let end_timestamp = dataset.4;

        // convert csv data into stmdb files
        csv_to_stmdb(exchange.to_string(), symbol.to_string(), dataset_id, start_timestamp, end_timestamp);

        // load stmdb files and loop through them
        load_stmdb(exchange.to_string(), symbol.to_string());
    }
}