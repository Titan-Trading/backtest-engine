use std::{fs::read_to_string, collections::HashMap, hash::Hash};
use rlua::{Lua, UserData, UserDataMethods, MetaMethod, Value, Function, StdLib};

// do some full LUA testing for our plugins (indicator, strategy, etc)
// how we are going to have lua execute rust functions and vice versa (passing data in and returning it out)

#[derive(Clone)]
struct Env {
    name: String,
    value: String
}
impl Env {
    pub fn new(name: String, value: String) -> Env {
        Env {
            name,
            value
        }
    }
}
impl UserData for Env {
    // fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    // }
}

fn test_startegy_plugin(lua: &Lua) {
    let lua_script = read_to_string("strategy_plugin_test.lua").unwrap();
    let lua_script = format!("{}{}", "dofile(\"helpers.lua\")\n", lua_script);

    lua.context(|lua_ctx| {
        // setup function hooks for being called in lua scripts
        let env_constructor = lua_ctx.create_function(|_, (name, value): (String, String)| {
            Ok(Env::new(name, value))
        }).unwrap();
        lua_ctx.globals().set("env", env_constructor).unwrap();

        let indicator_constructor = lua_ctx.create_function(|_, (name, periods, interval, symbol, exchange): (String, i32, String, String, String)| {
            let mut i = Indicator::new(name, periods, interval, symbol, exchange);

            i.update(0.35);
            
            Ok(i)
        }).unwrap();
        lua_ctx.globals().set("indicator", indicator_constructor).unwrap();
        
        let input_function = lua_ctx.create_function(|_, (name, datatype, default_value): (String, String, String)| {
            Ok("true")
        }).unwrap();
        lua_ctx.globals().set("input", input_function).unwrap();

        let get_position_function = lua_ctx.create_function(|_, (exchange, symbol): (String, String)| {
            Ok(false)
        }).unwrap();
        lua_ctx.globals().set("get_position", get_position_function).unwrap();

        let command_function = lua_ctx.create_function(|_, (command_id, command): (String, HashMap<String, String>)| {
            
            println!("perform command: {} {:?}", command_id, command);

            let symbol = command.get("symbol").unwrap();
            println!("symbol: {}", symbol);
            
            Ok(false)
        }).unwrap();
        lua_ctx.globals().set("command", command_function).unwrap();

        // let env_function = lua_ctx.create_function(|_, (name, value): (String, String)| {
        //     Ok(())
        // }).unwrap();
        // lua_ctx.globals().set("env", env_function).unwrap();

        lua_ctx
            .load(&lua_script.clone())
            .set_name("strategy_plugin_chunk")
            .unwrap()
            .exec()
            .unwrap();


        let on_bar_function: Function = lua_ctx.globals().get("on_bar").unwrap();
    
        let bar_t = lua_ctx.create_table().unwrap();
        bar_t.set("exchange", "NYSE").unwrap();
        bar_t.set("symbol", "SPY").unwrap();
        bar_t.set("interval", "1m").unwrap();
        bar_t.set("timestamp", 1932804982).unwrap();
        bar_t.set("open", 100.23).unwrap();
        bar_t.set("high", 100.23).unwrap();
        bar_t.set("low", 100.23).unwrap();
        bar_t.set("close", 100.23).unwrap();
        bar_t.set("volume", 65465.454633).unwrap();

        let history_t = lua_ctx.create_table().unwrap();
        let timestamp_t = lua_ctx.create_table().unwrap();
        timestamp_t.set(1, 1932804982).unwrap();
        let open_t = lua_ctx.create_table().unwrap();
        open_t.set(1, 100.23).unwrap();
        let high_t = lua_ctx.create_table().unwrap();
        high_t.set(1, 100.23).unwrap();
        let low_t = lua_ctx.create_table().unwrap();
        low_t.set(1, 100.23).unwrap();
        let close_t = lua_ctx.create_table().unwrap();
        close_t.set(1, 100.23).unwrap();
        let volume_t = lua_ctx.create_table().unwrap();
        volume_t.set(1, 100.23).unwrap();

        history_t.set("timestamp", timestamp_t).unwrap();
        history_t.set("open", open_t).unwrap();
        history_t.set("high", high_t).unwrap();
        history_t.set("low", low_t).unwrap();
        history_t.set("close", close_t).unwrap();
        history_t.set("volume", volume_t).unwrap();
        
        let is_updated = on_bar_function.call::<_, Value>((bar_t, history_t)).unwrap();
        if let Value::Boolean(true) = is_updated {
            println!("strategy updated successfully");
        }
    });
}


#[derive(Clone)]
struct Indicator {
    name: String,
    periods: i32,
    interval: String,
    symbol: String,
    exchange: String,
    values: Vec<f64>
}
impl Indicator {
    // create a new indicator instance
    pub fn new(name: String, periods: i32, interval: String, symbol: String, exchange: String) -> Indicator {
        Indicator {
            name,
            periods,
            interval,
            symbol,
            exchange,
            values: Vec::new()
        }
    }

    // update the latest indicator value
    pub fn update(&mut self, value: f64) {
        // let latest_value = self.values.first().unwrap();

        self.values.push(value);
    }
}
impl UserData for Indicator {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // adds a method to the instance (uses : syntax)
        methods.add_method("history", |_, ind, ()| {
            // let mag_squared = vec.0 * vec.0 + vec.1 * vec.1;

            // get last x values

            Ok(ind.values.clone())
        });

        methods.add_method("val", |_, ind, ()| {
            let val = Value::Number(*ind.values.get(0).unwrap());

            Ok(val)
        });

        // adds a function to the instance (uses . syntax)
        // does not have access to internal variables
        // methods.add_function("val", |_, ()| {
        //     Ok(1)
        // });

        // methods.add_meta_function(MetaMethod::Add, |_, (vec1, vec2): (Vec2, Vec2)| {
        //     Ok(Vec2(vec1.0 + vec2.0, vec1.1 + vec2.1))
        // });*/
    }
}

struct IndicatorUpdate {
    values: Option<HashMap<String, f64>>,
    value: f64,
}

fn test_indicator_plugin(lua: &Lua) {
    let lua_script = read_to_string("indicator_plugin_test.lua").unwrap();
    let lua_script = format!("{}{}", "dofile(\"helpers.lua\")\n", lua_script);

    lua.context(|lua_ctx| {
        // setup function hooks for being called in lua scripts
        let update_function = lua_ctx.create_function(|_, update: HashMap<String, f64>| {

            println!("{:?}", update);

            Ok(true)
        }).unwrap();
        lua_ctx.globals().set("update", update_function).unwrap();

        
        let input_function = lua_ctx.create_function(|_, (name, datatype, default_value): (String, String, String)| {
            Ok(14)
        }).unwrap();
        lua_ctx.globals().set("input", input_function).unwrap();

        // let env_function = lua_ctx.create_function(|_, (name, value): (String, String)| {
        //     Ok(())
        // }).unwrap();
        // lua_ctx.globals().set("env", env_function).unwrap();

        lua_ctx
            .load(&lua_script.clone())
            .set_name("indicator_plugin_chunk")
            .unwrap()
            .exec()
            .unwrap();

        
        let on_bar_function: Function = lua_ctx.globals().get("on_bar").unwrap();
        
        let bar_t = lua_ctx.create_table().unwrap();
        bar_t.set("exchange", "NYSE").unwrap();
        bar_t.set("symbol", "SPY").unwrap();
        bar_t.set("interval", "1m").unwrap();
        bar_t.set("timestamp", 1932804982).unwrap();
        bar_t.set("open", 100.23).unwrap();
        bar_t.set("high", 100.23).unwrap();
        bar_t.set("low", 100.23).unwrap();
        bar_t.set("close", 100.23).unwrap();
        bar_t.set("volume", 65465.454633).unwrap();

        let history_t = lua_ctx.create_table().unwrap();
        let timestamp_t = lua_ctx.create_table().unwrap();
        timestamp_t.set(1, 1932804982).unwrap();
        let open_t = lua_ctx.create_table().unwrap();
        open_t.set(1, 100.23).unwrap();
        let high_t = lua_ctx.create_table().unwrap();
        high_t.set(1, 100.23).unwrap();
        let low_t = lua_ctx.create_table().unwrap();
        low_t.set(1, 100.23).unwrap();
        let close_t = lua_ctx.create_table().unwrap();
        close_t.set(1, 100.23).unwrap();
        let volume_t = lua_ctx.create_table().unwrap();
        volume_t.set(1, 100.23).unwrap();

        history_t.set("timestamp", timestamp_t).unwrap();
        history_t.set("open", open_t).unwrap();
        history_t.set("high", high_t).unwrap();
        history_t.set("low", low_t).unwrap();
        history_t.set("close", close_t).unwrap();
        history_t.set("volume", volume_t).unwrap();
        
        let is_updated = on_bar_function.call::<_, Value>((bar_t, history_t)).unwrap();
        if let Value::Boolean(true) = is_updated {
            println!("indicator updated successfully");
        }
    });
}

fn main() {

    let std_lib_flags: StdLib = StdLib::BASE
    | StdLib::MATH
    | StdLib::STRING
    | StdLib::TABLE
    | StdLib::UTF8;

    let lua = Lua::new_with(std_lib_flags);

    test_indicator_plugin(&lua);
    test_startegy_plugin(&lua);
    
}