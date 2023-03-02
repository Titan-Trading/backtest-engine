// uses buffer streams from filesystem to read and write data
// supports csv and stmdb formats currently

use std::{io::{Error, BufReader}, collections::HashMap, fs::File};
use csv::ByteRecord;
use super::{types::{FileType, csv::CSVStreamReader, stmdb::{STMDBStreamReader, STMDBRecord, STMDBIndex}, json::JsonStreamReader}, storage::FileSystem};

// trait that defines the interface for reading files
pub trait FileStreamReader {
    type Output;
    
    // read a file into memory in chunks
    fn read_chunk(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error>;

    // read a file into memory all at once
    fn read_all(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error>;
}

#[derive(Debug)]
pub enum ReaderResult {
    ByteRecords(Vec<ByteRecord>), // csv
    String(String), // text
    STMDBRecords(Vec<STMDBRecord>), // stmdb
    STMDBIndex(STMDBIndex), // stmdb index
}

pub struct Reader {
    pub filetype: FileType,
    pub filesystem: FileSystem,
}

impl Reader {
    
    // create a new reader
    pub fn new() -> Reader {
       // connect to our filesystem
       let filesystem = FileSystem::connect();

        Reader {
            filetype: FileType::Text,
            filesystem,
        }
    }

    // create a new reader with a file type
    pub fn new_with(filetype: FileType) -> Reader {
        // connect to our filesystem
        let filesystem = FileSystem::connect();

        Reader {
            filetype,
            filesystem,
        }
    }
    
    // read a file into memory all at once
    pub fn read_file(&mut self, file_path: String, parameters: HashMap<String, String>) -> Result<ReaderResult, Error> {

        // determine the file type
        let path = file_path;

        // determine the file type
        let filetype = FileType::from_string(path.clone());

        // read the file stream
        let mut file_stream = match self.filesystem.read_stream(path.clone()) {
            Ok(stream) => stream,
            Err(err) => return Err(err),
        };

        // get filetype stream reader
        if filetype == FileType::Csv {
            // use csv stream reader to read all the contents of the file into records
            // parameters give us the ability to curse through the file
            let records = Box::new(CSVStreamReader::read_all(&mut file_stream, parameters)).unwrap();
            return Ok(ReaderResult::ByteRecords(records));
        }
        else if filetype == FileType::Stmdb {
            let records = Box::new(STMDBStreamReader::read_all(&mut file_stream, parameters)).unwrap();
            return Ok(ReaderResult::STMDBRecords(records));
        }
        else if filetype == FileType::Json {
            let records = Box::new(JsonStreamReader::read_all(&mut file_stream, parameters)).unwrap();
            return Ok(ReaderResult::STMDBIndex(records));
        }

        // read the file as text file
        Ok(ReaderResult::String(self.filesystem.read_file(path).unwrap()))
    }

    // read a file into memory in chunks, and return a stream/iterator
    pub fn read_chunk(&mut self, file_path: String, parameters: HashMap<String, String>) -> Result<ReaderResult, Error> {
        // determine the file type
        let path = file_path;

        // determine the file type
        let filetype = FileType::from_string(path.clone());

        // read the file stream
        let mut file_stream = match self.filesystem.read_stream(path.clone()) {
            Ok(stream) => stream,
            Err(err) => return Err(err),
        };

        // get filetype stream reader
        // parameters give us the ability to cursor/page through the file
        if filetype == FileType::Csv {
            // use csv stream reader to read a chunk of the contents of the file into records
            let records = Box::new(CSVStreamReader::read_chunk(&mut file_stream, parameters)).unwrap();
            return Ok(ReaderResult::ByteRecords(records));
        }
        else if filetype == FileType::Stmdb {
            // use stmdb stream reader to read a chunk of the contents of the file into records
            let records = Box::new(STMDBStreamReader::read_chunk(&mut file_stream, parameters)).unwrap();
            return Ok(ReaderResult::STMDBRecords(records));
        }
        else if filetype == FileType::Json {
            // use stmdb index stream reader to read a chunk of the contents of the file into records
            let records = Box::new(JsonStreamReader::read_chunk(&mut file_stream, parameters)).unwrap();
            return Ok(ReaderResult::STMDBIndex(records));
        }

        // read the file as text file
        // let records = Box::new(TextStreamReader::read_chunk(&mut file_stream, parameters)).unwrap();
        Ok(ReaderResult::String(self.filesystem.read_file(path).unwrap()))
        // Ok(ReaderResult::String(records))
    }
}