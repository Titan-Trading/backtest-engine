use std::{fs::File, io::{Error, BufReader, BufWriter}, collections::HashMap};
use csv::ByteRecord;
use crate::filesystem::{reader::FileStreamReader, writer::FileStreamWriter};
use super::stmdb::STMDBIndex;

pub struct JsonStreamReader {
    reader: BufReader<File>,
    // batch_size: usize,
    // has_header: bool,
    // delimiter: u8,
}

impl FileStreamReader for JsonStreamReader {
    type Output = STMDBIndex;

    // reads a chunk of records from the file using the current cursor position
    fn read_chunk(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error> {
        // let mut records = vec![];

        // read all the records
        let records: Self::Output = serde_json::from_reader(file_stream)?;

        Ok(records)
    }

    // reads the entire file into memory
    fn read_all(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error> {
        // let mut records = vec![];

        // read all the records
        let records: Self::Output = serde_json::from_reader(file_stream)?;

        Ok(records)
    }
}

pub struct JsonStreamWriter {
    writer: csv::Writer<BufWriter<File>>,
    // batch_size: usize,
    // has_header: bool,
    // delimiter: u8, 
}

impl FileStreamWriter for JsonStreamWriter {
    type Input = Vec<ByteRecord>;

    // write a chunk of records from the file using the current cursor position

    // writes the entire file
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