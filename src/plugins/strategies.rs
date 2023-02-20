use std::{io::{Error, Read}, fs, collections::HashMap};

use super::datasets::Dataset;

#[derive(Clone)]
pub struct Strategy {
    pub name: String,
    pub lua_script: String,
    pub settings: HashMap<String, String>,
}

impl Strategy {
    pub fn new(name: String, lua_script: String, settings: HashMap<String, String>) -> Strategy {
        Strategy {
            name: name,
            lua_script: lua_script,
            settings: settings,
        }
    }

    pub fn run(&self, datasets: Vec<Dataset>) {
        println!("running strategy: {}", self.name);
    }
}

// loads all strategies from the filesystem using the given path
pub fn load_strategies(strategies_path: String) -> Result<HashMap<String, Strategy>, Error> {
    let mut strategies = HashMap::new();
    for entry in fs::read_dir(&strategies_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let strategy_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let lua_file = format!("{}/{}{}", &strategies_path, strategy_name, "/strategy.lua");
            let settings_file = format!("{}/{}{}", &strategies_path, strategy_name, "/settings.txt");

            // load lua script file
            // println!("loading lua script: {}", lua_file);
            let mut file = fs::File::open(lua_file).unwrap();
            let mut lua_contents = String::new();
            file.read_to_string(&mut lua_contents).unwrap();

            // load settings file
            // println!("loading settings file: {}", settings_file);
            let mut file = fs::File::open(settings_file).unwrap();
            let mut settings_contents = String::new();
            file.read_to_string(&mut settings_contents).unwrap();

            let mut settings = HashMap::new();
            for line in settings_contents.lines() {
                let mut parts = line.splitn(2, '=');
                let key = parts.next().unwrap();
                let value = parts.next().unwrap();
                settings.insert(key.to_string(), value.to_string());
            }

            let strategy = Strategy::new(strategy_name.clone(), lua_contents, settings);

            strategies.insert(strategy_name, strategy);
        }
    }
    Ok(strategies)
}