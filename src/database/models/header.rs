use std::{io::{BufReader, Read, BufWriter}, fs::File};
use std::io::Error;

use byteorder::{BigEndian, WriteBytesExt};



pub struct Header {
    pub identifier: String, // STMDB (4 bytes)
    pub dataset_id: u32,
    pub start_timestamp: u32,
    pub end_timestamp: u32,
}

impl Header {
    // create a new empty header
    pub fn new() -> Self {
        Self {
            identifier: "STMDB".to_string(),
            dataset_id: 0,
            start_timestamp: 0,
            end_timestamp: 0,
        }
    }

    // use reader buffer to read the file header
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, Error> {
        let mut buffer = [0; 16];
        reader.read_exact(&mut buffer).unwrap();

        // println!("header buffer: {:?}", buffer);

        Ok(Self {
            identifier: u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]).to_string(),
            dataset_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            start_timestamp: u32::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
            end_timestamp: u32::from_be_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]),
        })
    }

    // write the header to a file buffer
    pub fn into_writer(writer: &mut BufWriter<File>, dataset_id: i32, start_timestamp: i32, end_timestamp: i32) -> Result<Self, Error> {
        // write file identifier (4 bytes)
        let filetype_id = "STMD";
        let filetype_id_bytes: [u8; 4] = match *filetype_id.as_bytes() {
            [b1, b2, b3, b4, ..] => [b1, b2, b3, b4],
            _ => panic!("filetype_id must be exactly 4 characters"),
        };
        writer.write_u32::<BigEndian>(u32::from_be_bytes(filetype_id_bytes)).unwrap();

        // write dataset id (4 bytes)
        writer.write_u32::<BigEndian>(dataset_id as u32).unwrap();

        // write start timestamp (4 bytes)
        writer.write_u32::<BigEndian>(start_timestamp as u32).unwrap();

        // write end timestamp (4 bytes)
        writer.write_u32::<BigEndian>(end_timestamp as u32).unwrap();

        Ok(Self {
            identifier: filetype_id.to_string(),
            dataset_id: dataset_id as u32,
            start_timestamp: start_timestamp as u32,
            end_timestamp: end_timestamp as u32,
        })
    }
}