use std::{fs::File, io::BufReader};
use std::io::{Error, ErrorKind};
use super::models::chunk::Chunk;
use super::models::header::Header;


// will read a chunk of data from a file using our .stmdb format
pub struct Reader {
    reader_buffer: BufReader<File>,
    has_header: bool,
}

impl Reader {
    // create a new reader
    pub fn new(filename: String) -> Self {
        // open the file
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                panic!("failed to open file: {:?}", e);
            }
        };

        // create a new reader
        let mut reader = BufReader::new(file);

        Self {
            has_header: false,
            reader_buffer: reader,
        }
    }

    // read the header of a stmdb file
    pub fn read_header(&mut self) -> Result<Header, Error> {
        // get a reference to the reader
        let mut reader = &mut self.reader_buffer;

        // seek to the beginning of the file
        match reader.seek_relative(0) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, format!("failed to seek to beginning of file: {}", e)));
            }
        }

        // read the header
        let header = Header::from_reader(&mut reader).unwrap();

        // set the has_header flag
        self.has_header = true;

        // return the header
        Ok(header)
    }
    
    // read a chunk of data from a stmdb file
    pub fn read_chunk(&mut self, limit: i32, offset: i32) -> Result<Chunk, Error> {
        // if header is not read, read it
        if !self.has_header {
            let header = self.read_header().unwrap();   
        }

        // get a reference to the reader
        let reader = &mut self.reader_buffer;

        // read a chunk from 
        let chunk = Chunk::from_reader(reader, limit, offset).unwrap();

        // return the chunk of data
        Ok(chunk)
    }
}