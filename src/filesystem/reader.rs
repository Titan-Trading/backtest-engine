// uses buffer streams from filesystem to read and write data
// supports csv and stmdb formats currently

use std::{io::{BufReader, Error}, fs::{File}};
use csv::ByteRecord;
use super::{types::{FileType, csv::CSVReader, stmdb::{STMDBReader, STMDBRecord}}, filesystem::FileSystem};

// trait that defines the interface for reading files
pub trait FileReader {
    type Output;
    
    // read a file into memory in chunks

    // read a file into memory all at once
    fn read_file(&mut self) -> Result<Self::Output, Error>;
}

#[derive(Debug)]
pub enum ReaderResult {
    ByteRecords(Vec<ByteRecord>),
    String(String),
    STMDBRecords(Vec<STMDBRecord>),
}

pub struct Reader {
    pub filetype: FileType,
    pub filesystem: FileSystem,
}

impl Reader {
    // create a new reader
    pub fn new(filetype: FileType) -> Reader {
       // connect to our filesystem
       let filesystem = FileSystem::connect();

        Reader {
            filetype,
            filesystem,
        }
    }

    // create a new reader with a file
    // pub fn new_with_file(file: File, filetype: FileType) -> Reader {
    //     let reader = match(filetype) {
    //         FileType::CSV => {
    //             Box::new(CSVReader::try_new(file))
    //         },
    //         FileType::STMDB => {
    //             Box::new(STMDBReader::try_new(file))
    //         },
    //         _ => {
    //             panic!("filetype not supported");
    //         }
    //     }

    //     Reader {
    //         filetype,
    //         reader: Box::new(file),
    //     }
    // }
    
    // read a file into memory all at once
    pub fn read_file(&mut self, file_path: String) -> Result<ReaderResult, Error> {

        let path = file_path.clone();

        // determine the file type
        let filetype = FileType::from_string(path.clone());

        // read the file
        let file_stream = self.filesystem.read_stream(path.clone());

        // get filetype reader
        if filetype == FileType::CSV {
            let reader = Box::new(CSVReader::try_new(file_stream));

            return Ok(ReaderResult::ByteRecords(reader.unwrap().read_file().unwrap()));
        } else if filetype == FileType::STMDB {
            let reader = Box::new(STMDBReader::try_new(file_stream));

            return Ok(ReaderResult::STMDBRecords(reader.unwrap().read_file().unwrap()));
        }

        // read the file as text file
        Ok(ReaderResult::String(self.filesystem.read_file(path.clone()).unwrap()))
    }

    // read a file into memory in chunks, and return a stream/iterator
    /*pub fn read(&mut self, file_path: String) -> Result<Vec<u8>, Error> {
        // determine the file type
        let filetype = FileType::from_string(file_path.clone());

        // get filetype reader

        let reader = match(filetype) {
            FileType::Text => {
                self.filesystem.read_file(file_path)
            },
            FileType::CSV => {
                Box::new(CSVReader::try_new(self.filesystem.read_stream(file_path)))
            },
            FileType::STMDB => {
                Box::new(STMDBReader::try_new(file))
            },
            _ => {
                panic!("filetype not supported");
            }
        };

        // read the file through a stream
        reader.read(file)
    }*/
}


/*pub struct Reader {
    pub filetype: FileType,
    pub reader: BufReader<File>,
}

impl Reader {
    // create a new reader
    pub fn new(buffer_reader: BufReader<File>, filetype: FileType) -> Reader {
        match(filetype) {
            FileType::CSV => {
                return Reader {
                    filetype,
                    reader: CSVReader::try_new(buffer_reader),
                }
            },
            FileType::STMDB => {
                return Reader {
                    filetype,
                    reader: STMDBReader::try_new(buffer_reader),
                }
            },
            _ => {
                panic!("filetype not supported");
            }
        }

        Reader {
            filetype,
            reader: buffer_reader,
        }
    }

    // create a new reader with a file
    pub fn new_with_file(file: File, filetype: FileType) -> Reader {
        match(filetype) {
            FileType::CSV => {
                return Reader {
                    filetype,
                    reader: CSVReader::try_new(file),
                }
            },
            FileType::STMDB => {
                return Reader {
                    filetype,
                    reader: STMDBReader::try_new(file),
                }
            },
            _ => {
                panic!("filetype not supported");
            }
        }

        Reader {
            filetype,
            reader: BufReader::new(file),
        }
    }

    // read a csv file into a vector of records
    // pub fn read_csv(&mut self) -> Vec<ByteRecord> {
    //     let mut csv_reader = csv::Reader::from_reader(self.reader.try_clone().unwrap());
    //     let mut records: Vec<ByteRecord> = Vec::new();

    //     for result in csv_reader.byte_records() {
    //         let record = result.unwrap();
    //         records.push(record);
    //     }

    //     records
    // }

    // read a stmdb file into a vector of records
    // pub fn read_stmdb(&mut self) -> Vec<ByteRecord> {
    //     let mut records: Vec<ByteRecord> = Vec::new();
    //     let mut byte_record = ByteRecord::new();
    //     let mut buffer = Vec::new();

    //     // read the first byte to get the number of fields
    //     self.reader.read(&mut buffer).unwrap();
    //     let field_count = buffer[0] as usize;

    //     // read the rest of the record
    //     self.reader.read_to_end(&mut buffer).unwrap();

    //     // loop through the buffer and parse the fields
    //     let mut field_index = 0;
    //     let mut field_length = 0;
    //     let mut field_start = 0;
    //     let mut field_end = 0;
    //     let mut field_buffer = Vec::new();

    //     for (index, byte) in buffer.iter().enumerate() {
    //         // skip the first byte
    //         if index == 0 {
    //             continue;
    //         }

    //         // if we are at the end of a field
    //         if field_index == field_count {
    //             // add the field to the record
    //             byte_record.push_field(&field_buffer);

    //             // reset the field buffer
    //             field_buffer.clear();

    //             // reset the field index
    //             field_index = 0;
    //         }
    //     }
    // }
}*/