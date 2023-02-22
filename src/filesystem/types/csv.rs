use std::{fs::File, io::{Error, BufReader, BufWriter}, sync::Arc};

use csv::ByteRecord;

use crate::filesystem::{reader::FileReader, writer::FileWriter};

pub struct CSVReader {
    reader: csv::Reader<BufReader<File>>,
    // batch_size: usize,
    // has_header: bool,
    // delimiter: u8,
}

impl CSVReader {
    pub fn try_new(
        file_stream: BufReader<File>
    ) -> Result<Self, Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(',' as u8)
            .from_reader(file_stream);

        Ok(Self {
            reader,
        })
    }
}

impl FileReader for CSVReader {
    type Output = Vec<ByteRecord>;

    // reads a chunk of records from the file using the current cursor position

    // reads the entire file into memory
    fn read_file(&mut self) -> Result<Self::Output, Error> {
        let mut records = vec![];

        // loop to read all the records in the chunk
        for record in self.reader.records() {
            let record = record?;
            records.push(record.into_byte_record());
        }

        Ok(records)
    }
}

pub struct CSVWriter {
    writer: csv::Writer<BufWriter<File>>,
    // batch_size: usize,
    // has_header: bool,
    // delimiter: u8, 
}

impl CSVWriter {
    pub fn try_new(file_stream: BufWriter<File>) -> Result<Self, Error> {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .delimiter(',' as u8)
            .from_writer(file_stream);

        Ok(Self {
            writer,
        })
    }
}

impl FileWriter for CSVWriter {
    type Input = Vec<ByteRecord>;

    // write a chunk of records from the file using the current cursor position

    // writes the entire file
    fn write_file(&mut self, input: Self::Input) -> Result<bool, Error> {
        
        for record in input {
            self.writer.write_byte_record(&record)?;
        }

        Ok(true)
    }
}