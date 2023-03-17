use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{Error, ErrorKind};
use mlua::{Lua, Value, Function as LuaFunction, FromLuaMulti, MultiValue, Result as LuaResult, ToLuaMulti, Error as LuaError};
use super::lua_hooks::LuaHook;


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

enum ScriptType {
    Strategy,
    Indicator,
}

pub struct Script {
    name: String,
    lua: Arc<Mutex<Lua>>,
    settings: HashMap<String, String>,
}

impl Script {
    pub fn new(name: String) -> Script {
        Script {
            name,
            lua: Arc::new(Mutex::new(Lua::new())),
            settings: HashMap::new(),
        }
    }

    // load a script into the lua context
    pub fn load(&mut self, script_type: ScriptType, script: String) -> Result<bool, Error> {
        let mut lua = self.lua.lock().unwrap();
        let settings = self.settings.clone();

        match script_type {
            ScriptType::Strategy => {
                // setup the lua hooks for the strategy
                // get setting values from settings.txt and pass into strategy
                let available_settings = settings.clone();
                LuaHook::new_external(&lua, "input", move |(name, data_type, default_value): (String, String, String)| {
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

                    Ok(true)
                });

                // allows the strategy to set certain environment settings
                LuaHook::new_external(&lua, "env", |(name, value): (String, EnvironmentVariableType)| {
                    // check if setting is available
                    println!("set environment setting: {} {:?}", name, value);
                    // set setting value



                    Ok(true)
                });

                // allows strategy to setup/get an indicator instance
                LuaHook::new_external(&lua, "indicator", |(name, periods, interval, symbol): (String, i32, String, String)| {
                    // check if indicator is already created
                    // create new indicator and add to indicator list
                    // - backpropagate up to 300 of the latest candles, if available (so our values are up-to-date)
                    // get latest indicator values
                    // return indicator values back to lua script



                    Ok(1)
                });

                // allows strategy to check if there is a position open
                LuaHook::new_external(&lua, "has_position", |(exchange, symbol): (String, String)| {
                    // check if there is a position open for the given exchange and symbol
                    // return true or false back to lua script

                    

                    Ok(1)
                });

                // allows strategy ability to tell system to execute a command
                LuaHook::new_external(&lua, "command", |(command_id, params): (String, ())| {
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
            },
            ScriptType::Indicator => {
                // setup the lua hooks for the indicator
            },
            _ => {
                println!("Unknown script type");
            }
        }

        // load the script into the lua context
        let result = lua
            .load(&script)
            .set_name(&self.name)
            .unwrap()
            .exec();
        if let Err(e) = &result {
            println!("Error loading lua script: {}", e);

            return Err(Error::new(ErrorKind::Other, "Error loading lua script"));
        }

        Ok(true)
    }

    // execute a function in the lua context
    pub fn exec(&mut self, function_name: String, args: Vec<Value>) -> Result<Value, Error> {
        let lua = self.lua.lock().unwrap();

        let lock_result = lua.globals().get(function_name).clone();
        if let Err(e) = lock_result {
            println!("Error getting lua function: {}", e);

            return Err(Error::new(ErrorKind::Other, "Error getting lua function"));
        }
        let func: LuaFunction = lock_result.unwrap();

        let call_result = func.call::<_, Value>(args);
        if let Err(e) = call_result {
            println!("Error executing lua script: {}", e);

            return Err(Error::new(ErrorKind::Other, "Error executing lua script"));
        }
        let result = call_result.unwrap();
        let ret_result = result.clone();

        Ok(ret_result)
    }
}