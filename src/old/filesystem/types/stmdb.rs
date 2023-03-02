use std::{io::{BufReader, Error, Read, BufWriter, Write}, fs::File, collections::HashMap};
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use serde::{Serialize, Deserialize};
use crate::filesystem::{reader::FileStreamReader, writer::FileStreamWriter};

// represents a single raw bar record
pub struct STMDBChunk {
    pub field_count: u8,
    pub fields: Vec<STMDBField>,
}

// represents a data type stored in a field
#[derive(Debug)]
pub enum STMDBFieldType {
    Int64,
    Float64,
    String,
}

// represents a single raw field in a bar record
#[derive(Debug)]
pub struct STMDBField {
    pub length: u8,
    pub field_type: STMDBFieldType,
    pub original_value: f64,
}

// represents a single compiled bar record
#[derive(Debug, Clone)]
pub struct STMDBRecord {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct STMDBIndex {
    // when the file was last updated
    pub last_updated: i32,

    pub file_map: HashMap<String, STMDBFile>,

    // specific instances of a file byte or cache offset
    // pub entries: HashMap<String, STMDBIndexEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct STMDBFile {
    pub dataset_id: u32,
    pub last_updated: i32,
    pub symbol: String,
    pub exchange: String,
    pub start_timestamp: i32,
    pub end_timestamp: i32,
    // pub file_size: i32,
    // pub filename: String,
}

#[derive(Debug)]
// gives us a way to point our cursor to a specific record
pub struct STMDBIndexEntry {
    pub timestamp: i32,
    pub offset: i32,
}
impl STMDBIndexEntry {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            offset: 0,
        }
    }

    pub fn new_with(timestamp: i32, offset: i32) -> Self {
        Self {
            timestamp,
            offset,
        }
    }
}

// represents the header of a stmdb file
pub struct STMDBHeader {
    pub identifier: String,
    pub dataset_id: u32,
    pub start_timestamp: u32,
    pub end_timestamp: u32,
}
impl STMDBHeader {
    
    // creates a new header from a reader
    pub fn try_new_read(reader: &mut BufReader<File>) -> Result<Self, Error> {
        let mut buffer = [0; 16];
        reader.read_exact(&mut buffer).unwrap();

        println!("header buffer: {:?}", buffer);

        Ok(Self {
            identifier: u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]).to_string(),
            dataset_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            start_timestamp: u32::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
            end_timestamp: u32::from_be_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]),
        })
    }

    // creates a new header and writes it to the file
    pub fn try_new_write(writer: &mut BufWriter<File>, dataset_id: i32, start_timestamp: i32, end_timestamp: i32) -> Result<Self, Error> {
        // let dataset_id = 1;
        // let start_timestamp = 2;
        // let end_timestamp = 3;

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

// gives us the ability to read a stmdb file
pub struct STMDBStreamReader {
    file_stream: BufReader<File>,
    header: STMDBHeader,
    index: STMDBIndexEntry,
}

impl FileStreamReader for STMDBStreamReader {
    type Output = Vec<STMDBRecord>;

    // reads a chunk of records from the file using the current cursor position
    fn read_chunk(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error> {

        // read the header
        let header = STMDBHeader::try_new_read(file_stream)?;
        
        // create a starting index
        let index = STMDBIndexEntry::new();

        let mut records = vec![];

        // loop to read all the records in the chunk
        loop {
            // detect the end of a file
            let field_count_result = file_stream.read_u8();
            if field_count_result.is_err() {
                break;
            }

            // read the number of fields from u8
            let field_count = field_count_result.unwrap();

            // read the fields
            let mut fields = vec![];
            for i in 0..field_count {

                // read the length of the field
                let length = file_stream.read_u8().unwrap();

                // read the field type
                let field_type = file_stream.read_u8().unwrap();

                // read the original value
                let original_value = file_stream.read_f64::<BigEndian>().unwrap();

                // create a new field
                let field = STMDBField {
                    length,
                    field_type: match field_type {
                        0 => STMDBFieldType::Int64,
                        1 => STMDBFieldType::Float64,
                        2 => STMDBFieldType::String,
                        _ => panic!("invalid field type"),
                    },
                    original_value,
                };

                // add the field to the list
                fields.push(field);
            }

            // create a new chunk
            let chunk = STMDBChunk {
                field_count,
                fields,
            };

            // create a new record
            let record = STMDBRecord {
                timestamp: chunk.fields[0].original_value as i64,
                open: chunk.fields[1].original_value,
                high: chunk.fields[2].original_value,
                low: chunk.fields[3].original_value,
                close: chunk.fields[4].original_value,
                volume: chunk.fields[5].original_value,
            };

            // add the record to the list
            records.push(record);
        }

        Ok(records)
    }

    // reads the entire file into memory
    fn read_all(file_stream: &mut BufReader<File>, parameters: HashMap<String, String>) -> Result<Self::Output, Error> {

        // read the header
        let header = STMDBHeader::try_new_read(file_stream)?;
        
        // create a starting index
        let index = STMDBIndexEntry::new();

        // loop to read all the records in the chunk
        let mut records = vec![];
        loop {
            // detect the end of a file
            let field_count_result = file_stream.read_u8();
            if field_count_result.is_err() {
                break;
            }

            // read the number of fields from u8
            let field_count = field_count_result.unwrap();

            // read the fields
            let mut fields = vec![];
            for i in 0..field_count {

                // read the length of the field
                let length = file_stream.read_u8().unwrap();

                // read the data type of the field
                let data_type = file_stream.read_u8().unwrap();

                // different data types
                let mut field_type: STMDBFieldType = STMDBFieldType::Int64;
                let mut value: f64 = 0.0;
                if data_type == 1 {
                    // read as i64
                    value = file_stream.read_i64::<BigEndian>().unwrap() as f64;

                    field_type = STMDBFieldType::Int64;
                }
                else if data_type == 2 {
                    // read as f64
                    value = file_stream.read_f64::<BigEndian>().unwrap();

                    field_type = STMDBFieldType::Float64;
                }

                fields.push(STMDBField {
                    length,
                    field_type,
                    original_value: value,
                });
            }

            // read fields into a record
            let record = STMDBRecord {
                timestamp: fields[0].original_value as i64,
                open: fields[1].original_value,
                high: fields[2].original_value,
                low: fields[3].original_value,
                close: fields[4].original_value,
                volume: fields[5].original_value,
            };

            // add the record to the list
            records.push(record);
        }

        Ok(records)
    }
}



// takes a BufWriter<File> and writes records to it
pub struct STMDBStreamWriter {
    file_stream: BufWriter<File>,
    header: STMDBHeader,
    index: STMDBIndexEntry,
}

// writes a single record to the file
fn write_record(file_stream: &mut BufWriter<File>, record: &STMDBRecord) -> Result<bool, Error> {

    let mut buffer = vec![];

    // write the number of fields
    buffer.write_u8(6u8).unwrap();

    // write the timestamp (size, type, value)
    buffer.write_u8(8u8).unwrap();
    buffer.write_u8(1u8).unwrap();
    buffer.write_i64::<BigEndian>(record.timestamp).unwrap();

    // write the open (size, type, value)
    buffer.write_u8(8u8).unwrap();
    buffer.write_u8(2u8).unwrap();
    buffer.write_f64::<BigEndian>(record.open).unwrap();

    // write the high (size, type, value)
    buffer.write_u8(8u8).unwrap();
    buffer.write_u8(2u8).unwrap();
    buffer.write_f64::<BigEndian>(record.high).unwrap();

    // write the low (size, type, value)
    buffer.write_u8(8u8).unwrap();
    buffer.write_u8(2u8).unwrap();
    buffer.write_f64::<BigEndian>(record.low).unwrap();

    // write the close (size, type, value)
    buffer.write_u8(8u8).unwrap();
    buffer.write_u8(2u8).unwrap();
    buffer.write_f64::<BigEndian>(record.close).unwrap();

    // write the volume (size, type, value)
    buffer.write_u8(8u8).unwrap();
    buffer.write_u8(2u8).unwrap();
    buffer.write_f64::<BigEndian>(record.volume).unwrap();

    // write the buffer to the file
    file_stream.write_all(&buffer).unwrap();

    Ok(true)
}

impl FileStreamWriter for STMDBStreamWriter {
    type Input = Vec<STMDBRecord>;

    fn write_all(file_stream: &mut BufWriter<File>, input: Self::Input, parameters: HashMap<String, String>) -> Result<bool, Error> {

        // get parameters for headers
        let dataset_id = parameters.get("dataset_id").unwrap().to_string().parse::<i32>().unwrap();
        let start_timestamp = parameters.get("start_timestamp").unwrap().to_string().parse::<i32>().unwrap();
        let end_timestamp = parameters.get("end_timestamp").unwrap().to_string().parse::<i32>().unwrap();

        // write the header
        STMDBHeader::try_new_write(file_stream, dataset_id, start_timestamp, end_timestamp)?;
        
        // create a starting index
        let index = STMDBIndexEntry::new();

        // write the records
        for record in input {
            write_record(file_stream, &record)?;
        }

        Ok(true)
    }
}