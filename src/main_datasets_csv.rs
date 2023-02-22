

// do some full data processing tests for our plugins (indicator, strategy, etc)
// - datasets are stored in csv format (open, high, low, close, volume)
// - datasets are named by their folder with a format [exchange]_[symbol] ex: NYSE_SPY
// - datasets are stored in the lowest interval (1m, minute data)
// - other interval bars are consolidated by the system
// -- consolidations can be 1m increments with 
// --- [x]m - any number of minutes
// --- [x]h - any number of hours
// --- [x]d - any number of days
// --- [x]w - any number of weeks
// --- [x]M - any number of months
// --- [x]y - any number of years 
// - we have optional data integrity checks like missing data, unable to sync, etc.
// - we have the optional ability to smooth out missing chunks of data
// - datasets sync'd up to be used within a strategy
// -- we sync the underlying dataset interators so our consolidation will be accurate
// - each strategy has a history which carries over into the indicators within that strategy as well
// -- history contains x number of previous bars
// -- history is limited per strategy with a history_limit setting
// -- history is limited by the platform at 1k (for shared resource reasons)
// - each strategy has a limit on how many datasets it can have (hard limit of 5)
// - that means a strategy can have 5 datasets, 5 intervals of each, 1000 bars (roughly 500KB of data)

use csv::{Reader, ReaderBuilder};
use chrono::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug)]
struct Row {
    timestamp: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

struct ReaderSet {
    readers: Vec<Reader<BufReader<File>>>,
}

impl ReaderSet {
    fn new(file_names: &[&str], has_headers: bool) -> io::Result<Self> {
        let mut readers = Vec::new();
        for file_name in file_names {
            let file = File::open(file_name)?;
            let reader = BufReader::with_capacity(1_000_000, file);
            let csv_reader = ReaderBuilder::new()
                .has_headers(has_headers)
                .from_reader(reader);

            readers.push(csv_reader);
        }
        Ok(Self { readers })
    }

    fn next_rows(&mut self) -> io::Result<Vec<Option<Row>>> {
        let mut rows = Vec::new();
        let mut timestamps = Vec::new();
        for reader in &mut self.readers {
            match reader.records().next() {
                Some(Ok(record)) => {
                    let timestamp = Utc
                        .datetime_from_str(&record[1], "%Y-%m-%d %H:%M:%S")
                        .unwrap();
                    let open = record[3].parse().unwrap();
                    let high = record[4].parse().unwrap();
                    let low = record[5].parse().unwrap();
                    let close = record[6].parse().unwrap();
                    let volume = record[7].parse().unwrap();
                    let row = Row {
                        timestamp,
                        open,
                        high,
                        low,
                        close,
                        volume,
                    };
                    rows.push(Some(row));
                    timestamps.push(timestamp);
                }
                Some(Err(e)) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                }
                None => {
                    rows.push(None);
                    timestamps.push(Utc.timestamp(0, 0));
                }
            }
        }
        let min_timestamp = timestamps.iter().min().unwrap();
        for (row, timestamp) in rows.iter_mut().zip(timestamps.iter()) {
            if *timestamp != *min_timestamp {
                *row = None;
            }
        }
        Ok(rows)
    }
}

fn main() {
    let has_headers = true;
    let file_names = &["KuCoin_DASHUSDT.csv", "KuCoin_ADAUSDT.csv", "KuCoin_BTCUSDT.csv"];
    let mut reader_set = ReaderSet::new(file_names, has_headers).unwrap();

    loop {
        let rows = reader_set.next_rows().unwrap();
        if rows.iter().all(|row| row.is_none()) {
            break;
        }

        let min_timestamp = rows
            .iter()
            .filter_map(|row| row.as_ref().map(|r| r.timestamp))
            .min()
            .unwrap();

        for row in rows {
            if let Some(row) = row {
                if row.timestamp == min_timestamp {
                    // Process the row here
                    // println!("{:?}", row);
                }
            }
            else {
                println!("Invalid row detected: {:?}", row);
            }
        }
    }
}