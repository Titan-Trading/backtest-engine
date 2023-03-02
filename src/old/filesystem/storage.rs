// This will handle how the filesystem is accessed low level
// It will be used to load datasets and other files
// Will be used to save logs, metrics, and other files
// Read and write STMD files (.stmdb)

use std::{fs::{File, read_to_string, OpenOptions}, io::{Error, BufReader, BufWriter, Write}};

pub struct FileSystem {
    pub cwd: String,
}

impl FileSystem {
    
    // connect to the filesystem starting at a given path
    pub fn connect() -> FileSystem {
        FileSystem {
            cwd: "./data".to_string()
        }
    }

    // set the current working directory
    // will be applied to all filesystem operations
    pub fn set_cwd(&mut self, path: String) {
        self.cwd = path;
    }

    // get a file from the filesystem (all at once, into memory)
    pub fn read_file(&self, file_path: String) -> Result<String, Error> {

        let path = format!("{}/{}", self.cwd, file_path);

        let contents = read_to_string(path).expect("Unable to read file");

        Ok(contents)
    }

    // create a read stream from the filesystem
    pub fn read_stream(&mut self, file_path: String) -> Result<BufReader<File>, Error> {
        let mut path = format!("{}/{}", self.cwd, file_path);

        match File::open(&mut path) {
            Ok(file) => {
                let mut file_stream = BufReader::new(file);
                Ok(file_stream)
            },
            Err(e) => {
                println!("Unable to open file \"{}\": {}", path, e);
                
                Err(e)
            },
        }
    }

    // write a file to the filesystem (all at once)
    pub fn write_file(&self, file_path: String, contents: String) -> Result<bool, Error> {
        let path = format!("{}/{}", self.cwd, file_path);
        
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("Unable to open file");

        file.write_all(contents.as_bytes()).expect("Unable to write file");

        Ok(true)
    }

    // create a write stream to the filesystem
    pub fn write_stream(&self, file_path: String) -> BufWriter<File> {
        let path = format!("{}/{}", self.cwd, file_path);

        let file = File::create(path).expect("Unable to create file");

        BufWriter::new(file)
    }
}