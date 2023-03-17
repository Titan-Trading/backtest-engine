use std::{collections::HashMap, sync::Arc};
use mlua::Lua;
use crate::{database::models::bar::Bar, datasets::dataset::Dataset};


// an instance of a indicator that has been started
#[derive(Clone)]
pub struct Indicator {
    pub name: String,
    pub lua_state: Arc<Lua>,
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
        let lua = Arc::new(Lua::new());

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
    pub fn update(&self, bar: &Bar) {
        println!("updating indicator: {}", self.name);
    }
}
