use std::{io::{Error, Read}, fs, collections::HashMap, any::Any};
use rlua::{Lua, Function, Variadic, MultiValue, Context, FromLuaMulti, ToLuaMulti, Value};
use crate::{models::{Order, Metric}, datasets::Dataset};
use super::{indicators::Indicator, lua_hooks::LuaHook};

enum SettingType {
    Integer(i64),
    Float(f64),
    String(String),
}


// holds details about an available strategy plugin
#[derive(Clone)]
pub struct StrategyPlugin {
    pub name: String,
    pub lua_script: String,
    pub settings: HashMap<String, String>,
}

impl StrategyPlugin {
    pub fn new(name: String, lua_script: String, settings: HashMap<String, String>) -> StrategyPlugin {
        StrategyPlugin {
            name: name,
            lua_script: lua_script,
            settings: settings,
        }
    }
}

// an instance of a strategy that has been started
pub struct Strategy {
    pub name: String,
    pub lua_script_hash: String,
    pub lua_state: Lua,
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

        let name = name.clone();
        let settings_clone = settings.clone();

        // create new Lua state
        let lua = Lua::new();

        // setup lua environment
        Self::setup_sandbox(&lua, lua_script.clone(), settings_clone).unwrap();

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
            settings: settings,
            datasets,
            orders: Vec::new(),
            pending_orders: Vec::new(),
            metrics: None
        }
    }

    // setup lua environment
    fn setup_sandbox(lua: &Lua, lua_script: String, settings: HashMap<String, String>) -> Result<(), Error> {

        // setup lua context
        // setup wrapper to control method calls from Rust into LUA or vice versa
        let lua_context = lua.context(|lua_ctx| {
            let get_setting_handle = lua_ctx.create_function(move |_, (name, data_type, default_value): (String, String, String)| {
                
                /*let value = match settings.get(&name) {
                    Some(setting_value) => {
                        let value: Option<SettingType> = match data_type.as_str() {
                            "integer" => {
                                let parsed_value = setting_value.parse::<i64>().unwrap();
                                Ok(SettingType::Integer(parsed_value))
                            },
                            "float" => {
                                let parsed_value = setting_value.parse::<f64>().unwrap();
                                Ok(SettingType::Float(parsed_value))
                            },
                            "string" => {
                                let parsed_value = setting_value.parse::<String>().unwrap();
                                return Ok(Value::(parsed_value))
                            },
                            _ => Err(format!("Invalid data type {} for setting {}", data_type, name)),
                        };

                        value.unwrap()
                    },
                    None => panic!("Invalid setting name {}", name),
                };*/

                /*let setting_value = settings_clone.get(&name).unwrap();

                match (data_type.as_str(), setting_value) {
                    ("integer", value) if value.parse::<i64>().is_ok() => {
                        let parsed_value = value.parse::<i64>().unwrap();
                        // Some(Box::new(parsed_value))

                        Ok(parsed_value)
                    },
                    ("float", value) if value.parse::<f64>().is_ok() => {
                        let parsed_value = value.parse::<f64>().unwrap();
                        // Some(Box::new(parsed_value))

                        Ok(parsed_value)
                    },
                    ("string", value) => {
                        let string_value = value.clone();
                        // Some(Box::new(string_value))

                        Ok(string_value)
                    },
                    _ => Err(format!("Invalid data type {} for setting {}", data_type, name)),
                }*/

                Ok(true)
            }).unwrap();
   

            // create a rust hook to be called from with lua script
            // LuaHook::new_external(&lua_ctx, "get_setting", |_, (name, data_type, default_value): (String, String, String)| {Ok(1)});
            // LuaHook::new_external(&lua_ctx, "indicator", |_, (name, periods, interval, symbol): (String, i32, String, String)| {
            //     // check if indicator is already created
            //     // create new indicator and add to indicator list
            //     // - backpropagate up to 300 of the latest candles (so our values are up-to-date)
            //     // get latest indicator values

            //     Ok(1)
            // });
            // LuaHook::new_external(&lua_ctx, "get_position", |_, (exchange, symbol): (String, String)| {Ok(1)});
            // LuaHook::new_external(&lua_ctx, "market_order", |_, (exchange, symbol, quantity, side): (String, String, f64, String)| {Ok(1)});
            // LuaHook::new_external(&lua_ctx, "limit_order", |_, (exchange, symbol, price, quantity, side): (String, String, f64, f64, String)| {Ok(1)});

            let result = lua_ctx.load(&lua_script.clone()).exec();
            if let Err(e) = result {
                println!("Error loading Lua script: {}", e);
                return;
            }

            // status, exchange, symbol, timestamp, open, high, low, close, volume
            let test_value: (String, String, String, i32, f64, f64, f64, f64);
            
            // called on new bar
            // LuaHook::call(&lua_ctx, "on_bar", MultiValue::from(test_value));

            // called on order update
            // LuaHook::call(&lua_ctx, "on_order", MultiValue::new());
        });

        // setup lua hooks
        // setup lua functions

        Ok(())
    }

    // 
    pub fn run(&self,) {
        // println!("running strategy: {}", self.name);

    }
}

// loads all strategy plugins from the filesystem using the given path
pub fn load_strategy_plugins(strategies_path: String) -> Result<HashMap<String, StrategyPlugin>, Error> {
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

            let strategy = StrategyPlugin::new(strategy_name.clone(), lua_contents, settings);

            strategies.insert(strategy_name, strategy);
        }
    }
    Ok(strategies)
}