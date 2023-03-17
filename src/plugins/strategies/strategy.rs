use std::{io::{Error, ErrorKind}, collections::HashMap, sync::{Arc, Mutex}};
use mlua::{Lua, Value, MultiValue, FromLuaMulti, Result as LuaResult, ToLuaMulti, Error as LuaError};
use crate::{models::{Order, Metric}, plugins::{indicators::indicator::Indicator, lua_hooks::LuaHook}, datasets::dataset::Dataset};


// different types of environment variables that can be used in a strategy sandbox
#[derive(Debug, Clone)]
enum EnvironmentVariableType {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
impl<'lua> FromLuaMulti<'lua> for EnvironmentVariableType {
    fn from_lua_multi(values: MultiValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let value = values.get(0).unwrap();
        match value {
            Value::Boolean(b) => Ok(EnvironmentVariableType::Boolean(*b)),
            Value::Integer(i) => Ok(EnvironmentVariableType::Integer(*i)),
            Value::Number(n) => Ok(EnvironmentVariableType::Float(*n)),
            Value::String(s) => Ok(EnvironmentVariableType::String(s.to_str().unwrap().to_string())),
            _ => Err(LuaError::FromLuaConversionError {
                from: "unknown",
                to: "EnvironmentVariableType",
                message: Some("unknown environment variable type".to_string()),
            }),
        }
    }
}
impl<'lua> ToLuaMulti<'lua> for EnvironmentVariableType {
    fn to_lua_multi(self, lua: &'lua Lua) -> LuaResult<MultiValue<'lua>> {
        match self {
            EnvironmentVariableType::Boolean(b) => Ok(MultiValue::from_vec(vec![Value::Boolean(b)])),
            EnvironmentVariableType::Integer(i) => Ok(MultiValue::from_vec(vec![Value::Integer(i)])),
            EnvironmentVariableType::Float(f) => Ok(MultiValue::from_vec(vec![Value::Number(f)])),
            EnvironmentVariableType::String(s) => Ok(MultiValue::from_vec(vec![Value::String(lua.create_string(&s).unwrap())])),
        }
    }
}

// different types of settings that can be used in a strategy
#[derive(Debug, Clone)]
enum SettingType {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

// an instance of a strategy that has been started
#[derive(Clone)]
pub struct Strategy {
    pub name: String,
    pub lua_script_hash: String,
    pub lua_state: Arc<Mutex<Lua>>,
    pub indicators: HashMap<String, Indicator>,
    pub datasets: HashMap<String, Dataset>,
    pub settings: HashMap<String, String>,
    pub orders: Vec<Order>,
    pub pending_orders: Vec<Order>,
    pub metrics: Option<Vec<Metric<()>>>,
}

impl Strategy {
    pub fn new(
        name: String,
        lua_script: String,
        settings: HashMap<String, String>,
        datasets: HashMap<String, Dataset>,
        // metrics: Option<Vec<Metric<()>>>,
    ) -> Strategy {
        // clone the name so it can be used in the strategy
        let name = name.clone();
        let settings_clone = settings.clone();

        // create new lua state
        let lua = Arc::new(Mutex::new(Lua::new()));

        // setup lua environment
        Self::setup_sandbox(&lua, &lua_script, &name, settings_clone).unwrap();

        // verify that the lua script has the required functions
        // start up lua context
        // extract the settings required from the LUA script
        // verify the required settings match up with the supplied settings from settings.txt
        // perform initialization tests (run through some dummy data to see how the script responds)
        // prepare synchronized datasets (by timestamp)

        Strategy {
            name: name,
            lua_script_hash: "test".to_string(),
            lua_state: lua,
            indicators: HashMap::new(),
            settings,
            datasets,
            orders: Vec::new(),
            pending_orders: Vec::new(),
            metrics: None
        }
    }

    // setup lua environment
    fn setup_sandbox(lua: &Mutex<Lua>, lua_script: &String, strategy_name: &String, settings: HashMap<String, String>) -> Result<(), Error> {
        // setup lua context
        // setup wrapper to control method calls from Rust into LUA or vice versa

        // create a rust hook to be called from with lua script
        let lua = lua.lock().unwrap();

        

        // load and execute the lua script
        // let result = lua
        //     .load(lua_script)
        //     .set_name(&strategy_name)
        //     .unwrap()
        //     .exec();
        // if let Err(e) = &result {
        //     println!("Error loading lua script: {}", e);

        //     return Err(Error::new(ErrorKind::Other, "Error loading lua script"));
        // }

        Ok(())
    }

    // start running the strategy session
    // - loop through all the datasets and update the indicators and the strategy on each bar
    pub fn run(&self) {
        println!("running strategy: {}", self.name);

        // loop through all the datasets
        // - update the indicators
        // - update the strategy
        // - update the metrics
        // - update the orders
        // - update the pending orders
        /*for (dataset_name, dataset) in &self.datasets {
            println!("dataset: {}", dataset_name);

            // through all the bars in the dataset
            for bar in dataset.barset.bars.clone() {
                // update the indicators
                for (indicator_name, indicator) in &self.indicators {
                    println!("indicator: {}", indicator_name);

                    // update the indicator
                    indicator.update(&bar);
                }

                // update the strategy
                // - pass in the bar data
                // - pass in the indicator values
                // - pass in the metrics values
                // - pass in the orders
                // - pass in the pending orders
                // - pass in the settings
                // - pass in the command results
                // - pass in the position status
                // - pass in the account balance
                // - pass in the account equity
                // - pass in the account margin
                // - pass in the account free margin
                // - pass in the account margin level
                // - pass in the account profit
                // - pass in the account profit percent
                // - pass in the account drawdown

                // create a table to pass into the lua script
                // exchange, symbol, timestamp, open, high, low, close, volume
                let table = self.lua_state.create_table().unwrap();
                table.set("exchange", bar.exchange).unwrap();
                table.set("symbol", bar.symbol).unwrap();
                table.set("interval", bar.interval).unwrap();
                table.set("timestamp", bar.timestamp).unwrap();
                table.set("open", bar.open).unwrap();
                table.set("high", bar.high).unwrap();
                table.set("low", bar.low).unwrap();
                table.set("close", bar.close).unwrap();
                table.set("volume", bar.volume).unwrap();

                // call on new bar
                LuaHook::call(&self.lua_state, "on_bar", Value::Table(table)).unwrap();
                
                // update the metrics
                // - pass in the bar data
                // - pass in the indicator values
                // - pass in the order updates
            }
        }*/
    }
}