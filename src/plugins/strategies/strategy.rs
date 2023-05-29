use std::{io::Error, collections::HashMap, sync::{Arc, Mutex}};
use chrono::{Utc, TimeZone};
use rlua::{Lua, Value, MultiValue, Result as LuaResult, ToLuaMulti, Context};
use crate::{models::{Order, Metric}, plugins::{indicators::indicator::Indicator, lua_hooks::LuaHook}, database::{models::{query_builder::QueryBuilder, query::Query, exchange::Exchange, symbol::Symbol, bar::Bar}, database::Database}};


// different types of environment variables that can be used in a strategy sandbox
/*#[derive(Debug, Clone)]
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
}*/

// different types of settings that can be used in a strategy
#[derive(Debug, Clone)]
enum SettingType {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
impl<'lua> ToLuaMulti<'lua> for SettingType {
    fn to_lua_multi(self, lua: Context<'lua>) -> LuaResult<MultiValue<'lua>> {
        match self {
            SettingType::Boolean(b) => Ok(MultiValue::from_vec(vec![Value::Boolean(b)])),
            SettingType::Integer(i) => Ok(MultiValue::from_vec(vec![Value::Integer(i)])),
            SettingType::Float(f) => Ok(MultiValue::from_vec(vec![Value::Number(f)])),
            SettingType::String(s) => Ok(MultiValue::from_vec(vec![Value::String(lua.create_string(&s).unwrap())])),
        }
    }
}

// an instance of a strategy that has been started
#[derive(Clone)]
pub struct Strategy {
    pub name: String,
    pub lua_script_hash: String,
    pub lua_state: Arc<Mutex<Lua>>,
    pub indicators: HashMap<String, Indicator>,
    pub database: Database,
    pub query: Query,
    pub settings: HashMap<String, String>,
    pub orders: Vec<Order>,
    pub pending_orders: Vec<Order>,
    pub metrics: Option<Vec<Metric<()>>>,
    pub stopped_flag: bool,
}

impl Strategy {
    pub fn new(
        name: String,
        lua_script: String,
        settings: HashMap<String, String>,
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
            database: Database::new(),
            query: Query::new(1 as u64),
            orders: Vec::new(),
            pending_orders: Vec::new(),
            metrics: None,
            stopped_flag: false,
        }
    }

    // setup lua environment
    fn setup_sandbox(lua: &Mutex<Lua>, lua_script: &String, strategy_name: &String, settings: HashMap<String, String>) -> Result<(), Error> {
        // setup lua context
        let lua = lua.lock().unwrap();

        lua.context(|lua_ctx| {
            // setup the lua hooks for the strategy

            // get setting values from settings.txt and pass into strategy
            let available_settings = settings.clone();
            LuaHook::new_external(&lua_ctx, "input", move |(name, data_type, default_value): (String, String, String)| {
                // check if setting is available
                let value = match available_settings.get(&name) {
                    Some(setting_value) => {
                        // setting is available, use the value from settings.txt
                        let value: Option<SettingType> = match data_type.as_str() {
                            "boolean" => {
                                let parsed_value = setting_value.parse::<bool>().unwrap();
                                Some(SettingType::Boolean(parsed_value))
                            },
                            "integer" => {
                                let parsed_value = setting_value.parse::<i64>().unwrap();
                                Some(SettingType::Integer(parsed_value))
                            },
                            "float" => {
                                let parsed_value = setting_value.parse::<f64>().unwrap();
                                Some(SettingType::Float(parsed_value))
                            },
                            "string" => {
                                let parsed_value = setting_value.parse::<String>().unwrap();
                                Some(SettingType::String(parsed_value))
                            },
                            _ => {
                                panic!("Invalid data type {} - {}", data_type, name);
                            },
                        };

                        value.unwrap()
                    },
                    None => {
                        // setting is not available, use the default value
                        let value: Option<SettingType> = match data_type.as_str() {
                            "boolean" => {
                                let parsed_value = default_value.parse::<bool>().unwrap();
                                Some(SettingType::Boolean(parsed_value))
                            },
                            "integer" => {
                                let parsed_value = default_value.parse::<i64>().unwrap();
                                Some(SettingType::Integer(parsed_value))
                            },
                            "float" => {
                                let parsed_value = default_value.parse::<f64>().unwrap();
                                Some(SettingType::Float(parsed_value))
                            },
                            "string" => {
                                let parsed_value = default_value.parse::<String>().unwrap();
                                Some(SettingType::String(parsed_value))
                            },
                            _ => {
                                panic!("Invalid data type {} - {}", data_type, name);
                            },
                        };

                        value.unwrap()
                    }
                };

                println!("get input setting: {} {} {:?}", name, data_type, value);

                Ok(value)
            });

            // allows the strategy to set certain environment settings
            LuaHook::new_external(&lua_ctx, "env", |(name, value): (String, String)| {
                // check if setting is available
                println!("set environment setting: {} {:?}", name, value);
                // set setting value



                Ok(())
            });

            // allows strategy to setup/get an indicator instance
            LuaHook::new_external(&lua_ctx, "indicator", |(name, periods, interval, symbol): (String, i32, String, String)| {
                // check if indicator is already created
                // create new indicator and add to indicator list
                // - backpropagate up to 300 of the latest candles, if available (so our values are up-to-date)
                // get latest indicator values
                // return indicator values back to lua script



                Ok(1.00)
            });

            // allows strategy to check if there is a position open
            LuaHook::new_external(&lua_ctx, "has_position", |(exchange, symbol): (String, String)| {
                // check if there is a position open for the given exchange and symbol
                // return true or false back to lua script

                

                Ok(1)
            });

            // allows strategy ability to tell system to execute a command
            LuaHook::new_external(&lua_ctx, "command", |(command_id, params): (String, ())| {
                // check if command is valid
                // execute command
                // return true or false back to lua script

                if command_id == "order" {
                    println!("order command");
                }
                else if command_id == "insight" {
                    println!("insight command");
                }
                else {
                    println!("unknown command");
                }

                Ok(1)
            });

            // load and execute the lua script
            let result = lua_ctx
                .load(lua_script)
                .set_name(&strategy_name)
                .unwrap()
                .exec();
            if let Err(e) = &result {
                println!("Error loading lua script: {}", e);
            }
        });

        Ok(())
    }

    pub fn initialize(&mut self) {
        // run the strategy initialization
        // - run through some dummy data to see how the script responds
        // - prepare synchronized datasets (by timestamp)
        
        // get the start and end time from the settings
        let start_date = self.settings.get("start_date").unwrap();
        let start_timestamp = Utc.datetime_from_str(format!("{} {}", start_date, "00:00:00").as_str(), "%Y-%m-%d %H:%M:%S").unwrap().timestamp();
        let end_date = self.settings.get("end_date").unwrap();
        let end_timestamp = Utc.datetime_from_str(format!("{} {}", end_date, "23:59:59").as_str(), "%Y-%m-%d %H:%M:%S").unwrap().timestamp();
        
        // get the symbols from the settings
        let symbols = self.settings.get("datasets").unwrap().split(",")
            .map(|pair| {
                let mut parts = pair.split(":");
                let exchange = parts.next().unwrap();
                let symbol = parts.next().unwrap();
                let mut symbol_parts = symbol.split("-");
                let base_currency = symbol_parts.next().unwrap();
                let target_currency = symbol_parts.next().unwrap();

                (Exchange::new_with(exchange.to_string()), Symbol::new_with(target_currency.to_string(), base_currency.to_string()))
            })
            .collect::<Vec<(Exchange, Symbol)>>();

        // get the intervals from the settings
        let intervals = self.settings.get("intervals").unwrap().split(",")
            .map(|interval| {
                String::from(interval.to_string())
            })
            .collect::<Vec<String>>();
        
        // setup datasets/build query
        self.query = QueryBuilder::new(1 as u64)
            .with_limit(1000)
            .with_start_time(start_timestamp)
            .with_end_time(end_timestamp)
            .with_symbols(symbols)
            .with_intervals(intervals)
            .get();
    }

    // start running the strategy session
    // - loop through all the datasets and update the indicators and the strategy on each bar
    pub fn start(&mut self) {
        println!("running strategy: {}", self.name);

        // start the query
        let query_handle = self.database.start_query(1 as u64, self.query.clone());
        if let Err(e) = &query_handle {
            println!("Error starting query: {}", e);
        }
        let query_handle = query_handle.unwrap();
        let query_id = query_handle.query_id;

        // loop through until the query completes
        loop {

            // query a chunk of data
            let chunk_handle = self.database.query_chunk(query_id.clone(), HashMap::new());
            if let Err(e) = &chunk_handle {
                println!("Error with query chunk: {}", e);
            }
            let mut chunk_handle = chunk_handle.unwrap();

            // check if the stopped flag has been set
            if self.stopped_flag {
                break;
            }

            // get the next result
            let result = chunk_handle.next();

            // get the result
            let bar = result.unwrap();
            println!("bar: {:?}", bar);

            // update the indicators
            for (indicator_name, indicator) in &self.indicators {
                println!("indicator: {}", indicator_name);

                // update the indicator
                indicator.update(&bar);
            }

            // update the strategy
            self.update(&bar);
            
        }
    }

    // stop running the strategy session
    pub fn stop(&mut self) {
        println!("stopping strategy: {}", self.name);

        // stop the query
        self.stopped_flag = true;
    }

    pub fn update(&mut self, bar: &Bar) {
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
        let lua_state = self.lua_state.lock().unwrap();
        lua_state.context(|lua_ctx| {

            let query = self.query.clone();

            // loop through each candlestick within the current bar
            for (index, candlestick) in bar.candlesticks.bars.iter() {

                // loop through each exchange symbol combo
                for (exchange, symbol) in &query.symbols {
                    let exchange = exchange.clone();
                    let symbol = symbol.clone();

                    // loop through each interval
                    for interval in &query.intervals {
                        let interval = interval.clone();

                        // create a table to pass into the lua script
                        // exchange, symbol, interval, timestamp, open, high, low, close, volume
                        let table = lua_ctx.create_table().unwrap();
                        table.set("exchange", exchange.name.clone()).unwrap();
                        table.set("symbol", symbol.name.clone()).unwrap();
                        table.set("interval", interval).unwrap();
                        table.set("timestamp", candlestick.timestamp).unwrap();
                        table.set("open", candlestick.open).unwrap();
                        table.set("high", candlestick.high).unwrap();
                        table.set("low", candlestick.low).unwrap();
                        table.set("close", candlestick.close).unwrap();
                        table.set("volume", candlestick.volume).unwrap();

                        let params = vec![Value::Table(table)];

                        // call on new candlestick
                        LuaHook::call(&Box::new(lua_ctx), &"on_candlestick".to_string(), params).unwrap();
                    }
                }
            }
        });
        
        // update the metrics
        // - pass in the bar data
        // - pass in the indicator values
        // - pass in the order updates
    }
}