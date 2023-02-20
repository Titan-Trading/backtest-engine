use std::{io::Error, fs, collections::HashMap};

#[derive(Clone)]
pub struct Dataset {
    pub name: String,
    pub exchange: String,
    pub symbol: String,
    pub files: Vec<String>,
}

impl Dataset {
    pub fn new(name: String) -> Dataset {
        let parts = name.split('_').collect::<Vec<&str>>();
        let exchange = parts[0].to_string();
        let symbol = parts[1].to_string();

        Dataset {
            name: name,
            exchange: exchange,
            symbol: symbol,
            files: Vec::new(),
        }
    }
}


// loads all datasets from the filesystem using the given path
pub fn load_datasets(path: String) -> Result<HashMap<String, Dataset>, Error> {
    let mut datasets = HashMap::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dataset_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let mut dataset = Dataset::new(dataset_name.clone());

            // load files in directory
            for file_entry in fs::read_dir(path)? {
                let file_entry = file_entry?;
                let file_path = file_entry.path();
                if file_path.is_file() {
                    dataset.files.push(file_path.file_name().unwrap().to_str().unwrap().to_string());
                }
            }

            datasets.insert(dataset_name, dataset);
        }
    }
    Ok(datasets)
}