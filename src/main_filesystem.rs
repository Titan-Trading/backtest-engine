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

use filesystem::reader::Reader;
use filesystem::types::FileType;

use crate::filesystem::reader::ReaderResult;
use crate::filesystem::types::stmdb::STMDBRecord;
use crate::filesystem::writer::Writer;

mod filesystem;

fn csv_to_stmdb(exchange: String, symbol: String, dataset_id: i32, start_timestamp: i32, end_timestamp: i32) {

    let csv_file = format!("{}_{}.csv", exchange, symbol);
    let stmdb_file = format!("{}_{}_{}.stmdb", exchange, symbol, dataset_id);

    println!("converting {} to {}", csv_file, stmdb_file);

    let records = match Reader::new(FileType::CSV).read_file(csv_file.clone()) {
        Ok(ReaderResult::ByteRecords(records)) => records,
        Ok(ReaderResult::String(records)) => panic!("expected byte records, got string records"),
        Ok(ReaderResult::STMDBRecords(records)) => panic!("expected byte records, got stmdb records"),
        Err(e) => panic!("failed to read csv file: {}", e),
    };

    let mut stmdb_records: Vec<STMDBRecord> = vec![];
    for record in records {
        let stmdb_record = STMDBRecord {
            timestamp: String::from_utf8(record.get(0).ok_or("Missing timestamp field").unwrap().to_vec()).unwrap().parse::<i32>().unwrap(),
            open:   String::from_utf8(record.get(3).ok_or("Missing open field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            high:   String::from_utf8(record.get(4).ok_or("Missing high field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            low:    String::from_utf8(record.get(5).ok_or("Missing low field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            close:  String::from_utf8(record.get(6).ok_or("Missing close field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
            volume: String::from_utf8(record.get(7).ok_or("Missing volume field").unwrap().to_vec()).unwrap().parse::<f64>().unwrap(),
        };
        stmdb_records.push(stmdb_record);
    }

    let written = Writer::new(FileType::STMDB).write_records(stmdb_file.clone(), stmdb_records).expect("failed to write stmdb file");
    if written {
        println!("successfully wrote stmdb file");
    } else {
        println!("failed to write stmdb file");
    }
}

fn load_stmdb(exchange: String, symbol: String, dataset_id: i32) {

    let csv_file = format!("TEST_{}_{}.csv", exchange, symbol);
    let stmdb_file = format!("{}_{}_{}.stmdb", exchange, symbol, dataset_id);

    println!("converting {} to {}", stmdb_file, csv_file);

    let records = match Reader::new(FileType::STMDB).read_file(stmdb_file.clone()) {
        Ok(ReaderResult::ByteRecords(records)) => panic!("expected stmdb records, got byte records"),
        Ok(ReaderResult::String(records)) => panic!("expected stmdb records, got string records"),
        Ok(ReaderResult::STMDBRecords(records)) => records,
        Err(e) => panic!("failed to read stmdb file: {}", e),
    };

    let written = Writer::new(FileType::CSV).write_records(csv_file.clone(), records).expect("failed to write csv file");
    if written {
        println!("successfully wrote csv file");
    } else {
        println!("failed to write csv file");
    }
}

fn main() {

    // dataset details
    let exchange = "KuCoin";
    let symbol = "DASHUSDT";
    let dataset_id = 1;
    let start_timestamp = 1609459140;
    let end_timestamp = 1577836860;

    // convert csv data into stmdb files
    csv_to_stmdb(exchange.to_string(), symbol.to_string(), dataset_id, start_timestamp, end_timestamp);

    // load stmdb files and loop through them
    load_stmdb(exchange.to_string(), symbol.to_string(), dataset_id);

}