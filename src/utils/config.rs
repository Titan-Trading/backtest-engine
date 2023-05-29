use std::{fs, io::Read};

// Config is a struct that holds the configuration for the program
#[derive(Clone)]
pub struct Config {
    // The name of the path to the data csv files to read
    pub datasets_path: String,

    // The name of the path to indicator lua files
    pub indicators_path: String,

    // The name of the path to the strategy lua files and session settings to read
    pub strategies_path: String,
}

impl Config {
    // new() creates a new Config struct with default values
    pub fn new(datasets_path: String, indicators_path: String, strategies_path: String) -> Config {
        Config {
            datasets_path,
            indicators_path,
            strategies_path
        }
    }

    // set() sets a configuration value
    fn set(&mut self, key: &str, value: &str) {
        match key {
            "datasets_path" => self.datasets_path = value.to_string(),
            "indicators_path" => self.indicators_path = value.to_string(),
            "strategies_path" => self.strategies_path = value.to_string(),
            _ => println!("Unknown key: {}", key),
        }
    }
}

// read a config file and return a Config struct
pub fn read_config(path: String) -> Config {
    let mut config = Config::new("datasets".to_string(), "indicators".to_string(), "strategies".to_string());

    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        config.set(key, value);
    }
    config
}

// read a csv file and return a Vec of Vec of f64
pub fn read_data(path: String) -> Vec<Vec<f64>> {
    let mut data = Vec::new();

    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    for line in contents.lines().skip(1) {
        let mut row = Vec::new();
        for value in line.split(',') {
            row.push(value.parse::<f64>().unwrap());
        }
        data.push(row);
    }
    data
}
