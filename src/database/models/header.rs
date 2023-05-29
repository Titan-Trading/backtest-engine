use std::{io::{Read, Write, Seek, SeekFrom}, fs::File};
use std::io::Error;

use byteorder::{BigEndian, ByteOrder};

// constants
const IDENTIFIER: &str = "STMDB";
const HEADER_SIZE: usize = 24; // 24 bytes

pub struct Header {
    pub identifier: String, // STMDB (4 bytes)
    pub file_id: u32,
    pub start_timestamp: u64, // 8 bytes
    pub end_timestamp: u64,
}

impl Header {
    // create a new empty header
    pub fn new(file_id: u32, start_timestamp: u64, end_timestamp: u64) -> Self {
        Self {
            identifier: IDENTIFIER.to_owned(),
            file_id,
            start_timestamp,
            end_timestamp,
        }
    }

    // create a new header using an existing file instance
    pub fn from_file(file: &mut File) -> Result<Self, Error> {
        // create buffer for header
        let mut buffer = [0; HEADER_SIZE];

        // read header data into buffer (24 bytes)
        file.read(&mut buffer).unwrap();

        // read file identifier (4 bytes)
        let value = BigEndian::read_u32(&buffer[0..4]);
        let identifier = String::from_utf8_lossy(&value.to_be_bytes()).to_string();

        // create header
        let file_id         = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let start_timestamp = u64::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]);
        let end_timestamp   = u64::from_be_bytes([buffer[16], buffer[17], buffer[18], buffer[19], buffer[20], buffer[21], buffer[22], buffer[23]]);

        Ok(Self {
            identifier,
            file_id,
            start_timestamp,
            end_timestamp
        })
    }

    // write the header to a file
    pub fn into_writer(&mut self, file_instance: &mut File) -> Result<bool, Error> {
        // create buffer for header
        let mut buffer = [0; HEADER_SIZE];
        
        // write file identifier (4 bytes)
        let filetype_id = self.identifier.to_owned();
        let id_bytes = filetype_id.as_bytes();
        BigEndian::write_u32(&mut buffer, u32::from_be_bytes([id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]]));

        // write dataset id (4 bytes)
        buffer[4..8].copy_from_slice(&self.file_id.to_be_bytes());

        // write start timestamp (8 bytes)
        buffer[8..16].copy_from_slice(&self.start_timestamp.to_be_bytes());

        // write end timestamp (8 bytes)
        buffer[16..24].copy_from_slice(&self.end_timestamp.to_be_bytes());


        // one write call optimized, for less os syscalls
        let result = file_instance.write(&mut buffer);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        // check that the correct amount of bytes were written
        if result.unwrap() != HEADER_SIZE {
            return Err(Error::new(std::io::ErrorKind::Other, "Could not write header to file"));
        }

        // flush the file
        let result = file_instance.flush();
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        Ok(true)
    }

    // update an existing file header
    pub fn update(&mut self, file_handle: &mut File, file_id: u32, start_timestamp: u64, end_timestamp: u64) -> Result<bool, Error> {
        // update header
        self.file_id         = file_id;
        self.start_timestamp = start_timestamp;
        self.end_timestamp   = end_timestamp;

        // get current position in file
        let current_position = file_handle.seek(SeekFrom::Current(0)).unwrap();

        // seek to beginning of file
        file_handle.seek(SeekFrom::Start(0)).unwrap();

        // write header to file
        self.into_writer(file_handle).unwrap();
    
        // seek back to original position
        file_handle.seek(SeekFrom::Start(current_position)).unwrap();

        Ok(true)
    }
}