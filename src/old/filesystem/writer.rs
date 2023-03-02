// uses buffer streams from filesystem to read and write data
// supports csv and stmdb formats currently

use std::{io::{Error, BufWriter}, collections::HashMap, fs::File};
use csv::{ByteRecord};

use super::{types::{FileType, csv::CSVStreamWriter, stmdb::{STMDBStreamWriter, STMDBRecord}}, storage::FileSystem};

// trait that defines the interface for writing files
pub trait FileStreamWriter {
    type Input;
    
    // write a file in chunks

    // write a file all at once
    fn write_all(file_stream: &mut BufWriter<File>, input: Self::Input, parameters: HashMap<String, String>) -> Result<bool, Error>;
}

#[derive(Debug)]
pub enum WriterInput {
    ByteRecords(Vec<ByteRecord>),
    String(String),
    STMDBRecords(Vec<STMDBRecord>),
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
    pub fn write_file(&mut self, file_path: String, records: WriterInput, parameters: HashMap<String, String>) -> Result<bool, Error> {

        let path = file_path;

        // determine the file type
        let filetype = FileType::from_string(path.clone());

        // write the file
        let mut file_stream = self.filesystem.write_stream(path.clone());

        // handle the different input types
        if filetype == FileType::Csv {
            if let WriterInput::ByteRecords(byte_records) = records {
                let result = CSVStreamWriter::write_all(&mut file_stream, byte_records, parameters).unwrap();
                return Ok(result);
            } else {
                panic!("invalid input type for CSV file");
            }
        }
        else if filetype == FileType::Stmdb {
            if let WriterInput::STMDBRecords(stmdb_records) = records {
                let result = STMDBStreamWriter::write_all(&mut file_stream, stmdb_records, parameters).unwrap();
                return Ok(result);
            } else {
                panic!("invalid input type for STMDB file");
            }
        }
        else if filetype == FileType::Text {
            if let WriterInput::String(string) = records {
                let result = self.filesystem.write_file(path, string).unwrap();
                return Ok(result);
            } else {
                panic!("invalid input type for text file");
            }
        }

        panic!("filetype not supported")
    }
    // let result = match(records, filetype) {
    //     // csv
    //     (WriterInput::ByteRecords(byte_records), _file_type) => {
    //         return Ok(Box::new(CSVStreamWriter::write_all(file_stream, byte_records, parameters)))
    //     },
    //     // stmdb
    //     (WriterInput::STMDBRecords(stmdb_records), _file_type) => {
    //         let dataset_id = parameters.get("dataset_id").unwrap().parse::<i32>().unwrap();
    //         let start_timestamp = parameters.get("start_timestamp").unwrap().parse::<i32>().unwrap();
    //         let end_timestamp = parameters.get("end_timestamp").unwrap().parse::<i32>().unwrap();

    //         return Ok(Box::new(STMDBStreamWriter::write_all(file_stream, stmdb_records, parameters)))
    //     },
    //     (Err(err), _) => {
    //         panic!("filetype not supported");
    //     }
    // };

    // create a new writer with a file
    // pub fn new_with_file(file: File, filetype: FileType) -> Writer {
    //     Writer {
    //         filetype,
    //         writer: BufWriter::new(file)
    //     }
    // }
}