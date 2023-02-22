// uses buffer streams from filesystem to read and write data
// supports csv and stmdb formats currently

use std::{io::{BufWriter, Error}, fs::File};
use csv::{ByteRecord, StringRecord};

use super::{types::{FileType, csv::CSVWriter, stmdb::{STMDBWriter, STMDBRecord}}, filesystem::FileSystem};

// trait that defines the interface for writing files
pub trait FileWriter {
    type Input;
    
    // write a file in chunks

    // write a file all at once
    fn write_file(&mut self, input: Self::Input) -> Result<bool, Error>;
}


pub struct Writer {
    pub filetype: FileType,
    pub filesystem: FileSystem,
}

impl Writer {
    // create a new writer
    pub fn new(filetype: FileType) -> Writer {
        // connect to our filesystem
        let filesystem = FileSystem::connect();

        Writer {
            filetype,
            filesystem,
        }
    }

    // write records all at once
    pub fn write_records(&mut self, file_path: String, records: Vec<STMDBRecord>) -> Result<bool, Error> {

        let path = file_path.clone();

        // determine the file type
        let filetype = FileType::from_string(path.clone());

        // read the file
        let file_stream = self.filesystem.write_stream(path.clone());

        // get filetype reader
        if filetype == FileType::CSV {
            let writer = Box::new(CSVWriter::try_new(file_stream));

            let mut byte_records = vec![];
            for record in records {
                let mut string_record = StringRecord::new();
                string_record.push_field(record.timestamp.to_string().as_str());
                string_record.push_field(record.open.to_string().as_str());
                string_record.push_field(record.high.to_string().as_str());
                string_record.push_field(record.low.to_string().as_str());
                string_record.push_field(record.close.to_string().as_str());
                string_record.push_field(record.volume.to_string().as_str());

                let byte_record = ByteRecord::from(string_record);

                byte_records.push(byte_record);
            }

            return Ok(writer.unwrap().write_file(byte_records).unwrap());
        } else if filetype == FileType::STMDB {
            let writer = Box::new(STMDBWriter::try_new(file_stream));

            return Ok(writer.unwrap().write_file(records).unwrap());
        }

        // read the file as text file
        Ok(self.filesystem.write_file(path.clone(), "".to_string()).unwrap())
    }

    // create a new writer with a file
    // pub fn new_with_file(file: File, filetype: FileType) -> Writer {
    //     Writer {
    //         filetype,
    //         writer: BufWriter::new(file)
    //     }
    // }

    // write a vector of records to a csv file
    // pub fn write_csv(&mut self, records: Vec<ByteRecord>) {
    //     let mut csv_writer = csv::Writer::from_writer(self.writer.try_clone().unwrap());

    //     for record in records {
    //         csv_writer.write_byte_record(&record).unwrap();
    //     }
    // }

    // write stmdb file
    // pub fn write_stmdb(&mut self, records: Vec<ByteRecord>) {
    //     // write the number of fields
    //     self.writer.write(&[records[0].len() as u8]).unwrap();

    //     // write the records
    //     for record in records {
    //         for field in record.iter() {
    //             self.writer.write(&field).unwrap();
    //         }
    //     }
    // }
}