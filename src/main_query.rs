use std::{time::Instant, io::stdin, fs::read_to_string, collections::HashMap};
use mlua::{StdLib, Lua, Function, Value, UserDataMethods, UserData, LuaOptions};
use crate::database::{database::Database, models::{query::Query, exchange::Exchange, symbol::Symbol}};
mod database;


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

/*fn test_indicator_plugin(lua: &Lua) {
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
}*/

fn main() {
    // create a new lua instance
    let std_lib_flags: StdLib = StdLib::MATH
    | StdLib::STRING
    | StdLib::TABLE
    | StdLib::JIT;
    let lua = Lua::new_with(std_lib_flags, LuaOptions::default()).unwrap();
    // let lua = Lua::unsafe_new_with_flags(std_lib_flags, InitFlags::empty());
    // let jit = lua.exec::<_, bool>("return pcall(require, 'jit')").unwrap();

    // create a new database instance
    // initializes the underlying filesystem and processing engine
    // read the database index to get a layout of the partitioning of the datasets
    let database = Database::new();

    // create a new client
    let client_id = 1;

    // variables for performance testing
    let mut record_count = 0;
    let start = Instant::now();

    // read our strategy lua script from file and add our helper functions
    let lua_script = read_to_string("strategy_plugin_test.lua").unwrap();
    let lua_script = format!("{}{}", "dofile(\"helpers.lua\")\n", lua_script);

    // generate custom query to database
    let mut query: Query = Query::new(database.clone());
    let mut query_builder = Query::new_with(client_id, database)
        // defaults to 1000
        .with_limit(1000)

        // defaults to full datasets
        .with_start_time(1577836860)
        .with_end_time(1609459140)

        // limited to 5 intervals for now
        .with_intervals(Vec::from([
            "5m".to_string(),
            "15m".to_string(),
            "1h".to_string(),
            "4h".to_string(),
            "1d".to_string()
        ]))

        // limited to 5 symbols for now
        .with_symbols(Vec::from([
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("BTC".to_string(), "USDT".to_string())), 
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("ADA".to_string(), "USDT".to_string())),
            (Exchange::new_with("KuCoin".to_string()), Symbol::new_with("DASH".to_string(), "USDT".to_string()))
        ]));


    // setup function hooks for being called in lua scripts
    let env_constructor = lua.create_function(|_, (name, value): (String, String)| {
        Ok(Env::new(name, value))
    }).unwrap();
    lua.globals().set("env", env_constructor).unwrap();

    let indicator_constructor = lua.create_function(|_, (name, periods, interval, symbol, exchange): (String, i32, String, String, String)| {
        let mut i = Indicator::new(name, periods, interval, symbol, exchange);

        i.update(0.35);
        
        Ok(i)
    }).unwrap();
    lua.globals().set("indicator", indicator_constructor).unwrap();
    
    let input_function = lua.create_function(|_, (name, datatype, default_value): (String, String, String)| {
        Ok("true")
    }).unwrap();
    lua.globals().set("input", input_function).unwrap();

    let get_position_function = lua.create_function(|_, (exchange, symbol): (String, String)| {
        Ok(false)
    }).unwrap();
    lua.globals().set("get_position", get_position_function).unwrap();

    let command_function = lua.create_function(|_, (command_id, command): (String, HashMap<String, String>)| {
        
        // println!("perform command: {} {:?}", command_id, command);

        let symbol = command.get("symbol").unwrap();
        // println!("symbol: {}", symbol);
        
        Ok(false)
    }).unwrap();
    lua.globals().set("command", command_function).unwrap();

    // let env_function = lua_ctx.create_function(|_, (name, value): (String, String)| {
    //     Ok(())
    // }).unwrap();
    // lua_ctx.globals().set("env", env_function).unwrap();
        
    lua
        .load(&lua_script.clone())
        .set_name("strategy_plugin_chunk")
        .unwrap()
        .exec()
        .unwrap();

    // initialize the query (generate the id and start the tasks)
    query = query_builder.start();

    println!("starting query: {}", query.id.clone().unwrap());

    // get historical data from database
    // - when at least one barset is stored in the cache, the query will return
    // - next() can be called multiple times to get the next barset
    let mut last_timestamp = 0;
    while let Ok(result) = query.next() {
        if result.bars.len() == 0 {
            continue;
        }

        // track total records
        record_count += result.bars.len();

        // loop through the bars
        for bar in result.bars {
            if bar.timestamp < last_timestamp {
                println!("main: timestamp error: {} < {}", bar.timestamp, last_timestamp);
                return;
            }
            last_timestamp = bar.timestamp;

            // call the on_bar function in lua
            let on_bar_function: Function = lua.globals().get("on_bar").unwrap();
            let bar_t = lua.create_table().unwrap();
            bar_t.set("exchange", "KuCoin").unwrap();
            bar_t.set("symbol", "BTCUSDT").unwrap();
            bar_t.set("interval", "1m").unwrap();
            bar_t.set("timestamp", bar.timestamp).unwrap();
            bar_t.set("open", bar.candlesticks.by_symbol("KuCoin".to_string(), "BTCUSDT".to_string()).unwrap().open).unwrap();
            bar_t.set("high", bar.candlesticks.by_symbol("KuCoin".to_string(), "BTCUSDT".to_string()).unwrap().high).unwrap();
            bar_t.set("low", bar.candlesticks.by_symbol("KuCoin".to_string(), "BTCUSDT".to_string()).unwrap().low).unwrap();
            bar_t.set("close", bar.candlesticks.by_symbol("KuCoin".to_string(), "BTCUSDT".to_string()).unwrap().close).unwrap();
            bar_t.set("volume", bar.candlesticks.by_symbol("KuCoin".to_string(), "BTCUSDT".to_string()).unwrap().volume).unwrap();

            on_bar_function.call::<_, ()>((bar_t, Value::Nil), ).unwrap();
        }

        if result.status == "complete" {
            break;
        }
    }


    let on_bar_function: Function = lua.globals().get("on_bar").unwrap();

    let bar_t = lua.create_table().unwrap();
    bar_t.set("exchange", "NYSE").unwrap();
    bar_t.set("symbol", "SPY").unwrap();
    bar_t.set("interval", "1m").unwrap();
    bar_t.set("timestamp", 1932804982).unwrap();
    bar_t.set("open", 100.23).unwrap();
    bar_t.set("high", 100.23).unwrap();
    bar_t.set("low", 100.23).unwrap();
    bar_t.set("close", 100.23).unwrap();
    bar_t.set("volume", 65465.454633).unwrap();

    let history_t = lua.create_table().unwrap();
    let timestamp_t = lua.create_table().unwrap();
    timestamp_t.set(1, 1932804982).unwrap();
    let open_t = lua.create_table().unwrap();
    open_t.set(1, 100.23).unwrap();
    let high_t = lua.create_table().unwrap();
    high_t.set(1, 100.23).unwrap();
    let low_t = lua.create_table().unwrap();
    low_t.set(1, 100.23).unwrap();
    let close_t = lua.create_table().unwrap();
    close_t.set(1, 100.23).unwrap();
    let volume_t = lua.create_table().unwrap();
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

    println!("main: query id: {}", query.id.clone().unwrap());
    println!("main: symbols: {}", query.symbols.clone().len());
    println!("main: total results: {}", record_count);
    println!("main: query completed in {:?}s", (start.elapsed().as_millis() as f64 / 1000.0));
    
    // get live data from database
    
    // wait to exit
    println!("Press any key to continue");
    stdin().read_line(&mut String::new()).unwrap();   
}