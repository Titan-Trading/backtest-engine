use std::{io::{Error, Read}, fs, collections::HashMap};

use super::datasets::Dataset;

#[derive(Clone)]
pub struct Indicator {
    pub name: String,
    pub lua_script: String,
}

impl Indicator {
    pub fn new(name: String, lua_script: String) -> Indicator {
        Indicator {
            name: name,
            lua_script: lua_script,
        }
    }

    pub fn run(&self, datasets: Vec<Dataset>) {
        println!("running indicator: {}", self.name);
    }
}

// loads all indicators from the filesystem using the given path
pub fn load_indicators(indicators_path: String) -> Result<HashMap<String, Indicator>, Error> {
    let mut indicators = HashMap::new();
    for entry in fs::read_dir(&indicators_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let indicator_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let lua_file = format!("{}/{}{}", &indicators_path, indicator_name, ".lua");

            // load lua script file
            // println!("loading lua script: {}", lua_file);
            let mut file = fs::File::open(lua_file).unwrap();
            let mut lua_contents = String::new();
            file.read_to_string(&mut lua_contents).unwrap();

            let indicator = Indicator::new(indicator_name.clone(), lua_contents);

            indicators.insert(indicator_name, indicator);
        }
    }
    Ok(indicators)
}