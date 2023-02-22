// This will handle how the filesystem is accessed low level
// It will be used to load datasets and other files
// Will be used to save logs, metrics, and other files
// Read and write STMD files (.stmdb)

use std::{fs::{File, read_to_string, OpenOptions}, io::{Error, BufReader, BufWriter, Write}};

pub struct FileSystem {}

impl FileSystem {
    // connect to the filesystem starting at a given path
    pub fn connect() -> FileSystem {
        FileSystem {
        }
    }

    // get a file from the filesystem (all at once, into memory)
    pub fn read_file(&self, file_path: String) -> Result<String, Error> {
        let contents = read_to_string(file_path).expect("Unable to read file");

        Ok(contents)
    }

    // create a read stream from the filesystem
    pub fn read_stream(&self, file_path: String) -> BufReader<File> {
        let file = File::open(file_path).expect("Unable to open file");

        BufReader::new(file)
    }

    // write a file to the filesystem (all at once)
    pub fn write_file(&self, file_path: String, contents: String) -> Result<bool, Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path.clone())
            .expect("Unable to open file");

        file.write_all(contents.as_bytes()).expect("Unable to write file");

        Ok(true)
    }

    // create a write stream to the filesystem
    pub fn write_stream(&self, file_path: String) -> BufWriter<File> {
        let file = File::create(file_path).expect("Unable to create file");

        BufWriter::new(file)
    }
}