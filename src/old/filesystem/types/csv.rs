use std::{fs::File, io::{Error, BufReader, BufWriter}, collections::HashMap};

use csv::ByteRecord;

use crate::filesystem::{reader::FileStreamReader, writer::FileStreamWriter};

pub struct CSVStreamReader {
    reader: csv::Reader<BufReader<File>>,
    // batch_size: usize,
    // has_header: bool,
    // delimiter: u8,
}

impl FileStreamReader for CSVStreamReader {
    type Output = Vec<ByteRecord>;

    // reads a chunk of records from the file using the current cursor position
    fn read_chunk(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(file_stream);

        let mut records = vec![];

        // loop to read all the records in the chunk
        for record in reader.records() {
            let record = record?;
            records.push(record.into_byte_record());
        }

        Ok(records)
    }

    // reads the entire file into memory
    fn read_all(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error> {

        let should_reverse = parameters.get("reverse").unwrap_or(&"false".to_string()).parse::<bool>().unwrap();
        
        let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(file_stream);
        
        let mut records = vec![];

        // loop to read all the records in the chunk
        for record in reader.records() {
            let record = record?;
            records.push(record.into_byte_record());
        }

        if should_reverse {
            records.reverse();
        }

        Ok(records)
    }
}

pub struct CSVStreamWriter {
    writer: csv::Writer<BufWriter<File>>,
    // batch_size: usize,
    // has_header: bool,
    // delimiter: u8, 
}

impl FileStreamWriter for CSVStreamWriter {
    type Input = Vec<ByteRecord>;

    // write a chunk of records from the file using the current cursor position

    // writes the entire buffer to the file
    fn write_all(file_stream: &mut BufWriter<File>, input: Self::Input, parameters: HashMap<String, String>) -> Result<bool, Error> {

        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_writer(file_stream);
        
        for record in input {
            writer.write_byte_record(&record)?;
        }

        Ok(true)
    }
}