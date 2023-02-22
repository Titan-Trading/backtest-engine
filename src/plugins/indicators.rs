use std::{io::{Error, Read}, fs, collections::HashMap};

use rlua::Lua;

use crate::datasets::Dataset;

// use super::datasets::Dataset;

// holds details about an available indicator plugin
#[derive(Clone)]
pub struct IndicatorPlugin {
    pub name: String,
    pub lua_script: String,
}
impl IndicatorPlugin {
    pub fn new(name: String, lua_script: String) -> IndicatorPlugin {
        IndicatorPlugin {
            name: name,
            lua_script: lua_script,
        }
    }
}

// an instance of a indicator that has been started
pub struct Indicator {
    pub name: String,
    pub lua_state: Lua,
    pub settings: HashMap<String, String>,
    pub datasets: HashMap<String, Dataset>,
}

impl Indicator {
    pub fn new(
        name: String,
        settings: HashMap<String, String>,
        datasets: HashMap<String, Dataset>,
        // metrics: Option<Vec<Metric<()>>>,
    ) -> Indicator {

        // setup wrapper to control method calls from Rust into LUA or vice versa
        let lua = Lua::new();

        // verify that the lua script has the required functions
        // start up lua context
        // extract the settings required from the LUA script
        // verify the required settings match up with the supplied settings from settings.txt
        // perform initialization tests (run through some dummy data to see how the script responds)
        // prepare synchronized datasets (by timestamp)

        Indicator {
            name: name,
            lua_state: lua,
            settings: settings,
            datasets
        }
    }

    // 
    pub fn update(&self,) {
        println!("updating indicator: {}", self.name);
    }
}


// loads all indicators from the filesystem using the given path
pub fn load_indicators(indicators_path: String) -> Result<HashMap<String, IndicatorPlugin>, Error> {
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

            let indicator = IndicatorPlugin::new(indicator_name.clone(), lua_contents);

            indicators.insert(indicator_name, indicator);
        }
    }

    Ok(indicators)
}