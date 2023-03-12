use std::fs::File;
use std::io::Error;
use crate::database::models::chunk::Chunk;
use crate::database::models::header::Header;


// will read a chunk of data from a file using our .stmdb format
#[derive(Debug)]
pub struct Reader {
    // reader: BufReader<File>,
    file: File,
    has_header: bool,
}

impl Reader {
    // create a new reader
    pub async fn new(filename: String) -> Self {
        // open the file
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                panic!("failed to open file: {:?}", e);
            }
        };

        // get the file length
        let file_length = match file.metadata() {
            Ok(metadata) => metadata.len(),
            Err(e) => {
                panic!("failed to get file length: {:?}", e);
            }
        };

        // create a new reader 1GB chunks read into buffer at a time
        // let mut reader = BufReader::with_capacity(1024 * 1024, file);

        Self {
            has_header: false,
            file,
        }
    }

    pub fn new_with_file(file_instance: File) -> Self {
        Self {
            has_header: false,
            file: file_instance,
        }
    }

    // read the header of a stmdb file
    pub fn read_header(&mut self) -> Result<Header, Error> {
        // get a reference to the reader
        let mut file = &mut self.file;

        // read the header
        let header = Header::from_file(&mut file).unwrap();

        // set the has_header flag
        self.has_header = true;

        // return the header
        Ok(header)
    }
    
    // read a chunk of data from a stmdb file
    pub fn read_chunk(&mut self, limit: i64) -> Result<Chunk, Error> {
        // if header is not read, read it
        if !self.has_header {
            self.read_header().unwrap();
        }

        // get a reference to the reader
        let mut file = &mut self.file;

        // read a chunk from 
        let chunk = Chunk::from_file(file, limit).unwrap();

        // return the chunk of data
        Ok(chunk)
    }
}