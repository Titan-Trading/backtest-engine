pub mod csv;
// pub mod json;
// pub mod parquet;
pub mod stmdb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Text,
    CSV,
    STMDB,
}

impl FileType {
    // create a new filetype from a string
    pub fn from_string(filename: String) -> FileType {
        match filename.split('.').last().unwrap() {
            "text" => FileType::Text,
            "csv" => FileType::CSV,
            "stmdb" => FileType::STMDB,
            _ => panic!("filetype not supported"),
        }
    }
}