use std::{io::{BufReader, Error, Read, BufWriter, Write}, fs::File};
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

use crate::filesystem::{reader::FileReader, writer::FileWriter};


// represents the header of a stmdb file
pub struct STMDBHeader {
    pub identifier: String,
    pub dataset_id: u32,
    pub start_timestamp: u32,
    pub end_timestamp: u32,
}
impl STMDBHeader {
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

    pub fn try_new_write(writer: &mut BufWriter<File>) -> Result<Self, Error> {
        // let mut buffer = [0; 13];
        // buffer[0] = 0x53;
        // buffer[1] = 0x54;
        // buffer[2] = 0x4D;
        // buffer[3] = 0x44;
        // buffer[4] = 0x01;
        // buffer[5..9].copy_from_slice(&dataset_id.to_le_bytes());
        // buffer[9..13].copy_from_slice(&start_timestamp.to_le_bytes());
        // buffer[13..17].copy_from_slice(&end_timestamp.to_le_bytes());
        // writer.write_all(&buffer).unwrap();

        let dataset_id = 1;
        let start_timestamp = 2;
        let end_timestamp = 3;

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
            dataset_id,
            start_timestamp,
            end_timestamp,
        })
    }
}

// represents a single raw bar record
pub struct STMDBChunk {
    pub field_count: u8,
    pub fields: Vec<STMDBField>,
}

// represents a data type stored in a field
#[derive(Debug)]
pub enum STMDBFieldType {
    Int32,
    Float64,
}

// represents a single raw field in a bar record
#[derive(Debug)]
pub struct STMDBField {
    pub length: u8,
    pub field_type: STMDBFieldType,
    pub original_value: f64,
}

// represents a single compiled bar record
#[derive(Debug)]
pub struct STMDBRecord {
    pub timestamp: i32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

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

// gives us the ability to read a stmdb file
pub struct STMDBReader {
    file_stream: BufReader<File>,
    header: STMDBHeader,
    index: STMDBIndexEntry,
}

impl STMDBReader {
    pub fn try_new(mut file_stream: BufReader<File>) -> Result<Self, Error> {
        // read the header
        let header = STMDBHeader::try_new_read(&mut file_stream)?;
        
        // create a starting index
        let index = STMDBIndexEntry::new();

        Ok(Self {
            file_stream,
            header,
            index,
        })
    }
}

impl FileReader for STMDBReader {
    type Output = Vec<STMDBRecord>;

    // reads a chunk of records from the file using the current cursor position

    // reads the entire file into memory
    fn read_file(&mut self) -> Result<Self::Output, Error> {
        let mut records = vec![];

        // loop to read all the records in the chunk
        loop {
            // detect the end of a file
            let field_count_result = self.file_stream.read_u8();
            if field_count_result.is_err() {
                break;
            }

            // read the number of fields from u8
            let field_count = field_count_result.unwrap();
            // println!("field count: {}", field_count);

            // read the fields
            let mut fields = vec![];
            for i in 0..field_count {

                // read the length of the field
                let length = self.file_stream.read_u8().unwrap();
                // println!("field length: {}", field_count);

                // read the data type of the field
                let data_type = self.file_stream.read_u8().unwrap();
                // println!("data type: {}", data_type);

                // different data types
                let mut field_type: STMDBFieldType = STMDBFieldType::Int32;
                let mut value: f64 = 0.0;
                if data_type == 1 {
                    // read as i32
                    value = self.file_stream.read_i32::<BigEndian>().unwrap() as f64;

                    field_type = STMDBFieldType::Int32;
                }
                else if data_type == 2 {
                    // read as f64
                    value = self.file_stream.read_f64::<BigEndian>().unwrap();

                    field_type = STMDBFieldType::Float64;
                }

                fields.push(STMDBField {
                    length,
                    field_type: field_type,
                    original_value: value,
                });
            }

            // read fields into a record
            let record = STMDBRecord {
                timestamp: fields[0].original_value as i32,
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


pub struct STMDBWriter {
    file_stream: BufWriter<File>,
    header: STMDBHeader,
    index: STMDBIndexEntry,
}

impl STMDBWriter {
    pub fn try_new(mut file_stream: BufWriter<File>) -> Result<Self, Error> {
        // write the header
        let header = STMDBHeader::try_new_write(&mut file_stream)?;
        
        // create a starting index
        let index = STMDBIndexEntry::new();

        Ok(Self {
            file_stream,
            header,
            index,
        })
    }

    // writes a single record to the file
    pub fn write_record(&mut self, record: &STMDBRecord) -> Result<bool, Error> {

        let mut buffer = vec![];

        // write the number of fields
        buffer.write_u8(6u8).unwrap();

        // write the timestamp (size, type, value)
        buffer.write_u8(4u8).unwrap();
        buffer.write_u8(1u8).unwrap();
        buffer.write_i32::<BigEndian>(record.timestamp).unwrap();

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
        self.file_stream.write_all(&buffer).unwrap();

        Ok(true)
    }
}

impl FileWriter for STMDBWriter {
    type Input = Vec<STMDBRecord>;

    fn write_file(&mut self, input: Self::Input) -> Result<bool, Error> {

        // write the records
        for record in input {
            self.write_record(&record)?;
        }

        Ok(true)
    }
}