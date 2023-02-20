/*use rlua::{Lua, Result, Value, ToLua};

use crate::repositories::datasets::{load_datasets};
use crate::repositories::strategies::{load_strategies, Strategy};

mod utils;
mod models;
mod repositories;


fn main() {
    // Create a new Lua instance
    let lua = Lua::new();

    // Get the path to the config file from the command line
    let config_path = std::env::args().nth(1).unwrap_or("config.txt".to_string());

    // Read the config file into a Config struct
    let config = utils::read_config(config_path);

    // Load datasets
    // - get list of all folder in datasets folder
    // - for each folder, load the data.csv file into a Vec<Vec<f64>>
    // - store the Vec<Vec<f64>> in a HashMap<String, Vec<Vec<f64>>>
    let datasets = load_datasets(config.datasets_path).unwrap();

    // Load strategies
    // - get list of all folders in strategies folder
    // - for each folder, load the settings.txt file into a HashMap<String, String>
    // - store the HashMap<String, String> in a HashMap<String, HashMap<String, String>>
    // - for each folder, load the strategy.lua file into a String
    // - store the String in a HashMap<String, String>
    let strategies = load_strategies(config.strategies_path).unwrap();

    for strategy in &strategies {
        println!("strategy: {}", strategy.name);
        // println!("lua script: \n{}", strategy.lua_script);

        println!("settings:");
        for (key, value) in &strategy.settings {
            println!("{}: {}", key, value);
        }

        // check if the strategy has the right datasets
        let strategy_datasets = &strategy.settings.get_key_value("datasets").unwrap();
        strategy_datasets.1.split(",").for_each(|dataset_name| {
            let mut found = false;

            for dataset in &datasets {
                if dataset.name == dataset_name {
                    println!("found dataset: {}", dataset_name);

                    found = true;
                    break;
                }
            }

            if !found {
                panic!("could not find dataset: {}", dataset_name);
            }
        });

        fn get_setting_value<'lua>(lua_ctx: rlua::Context<'lua>, strategy: Strategy, key: String) -> Result<String> {
            let value = strategy.settings.get(&mut key).unwrap();
            Ok(value.to_string())
            // Ok("test".to_string())
        }


        // setup the lua script
        // - setup Rust functions that can be called from Lua
        // - setup Lua functions that can be called from Rust
        // - load the lua script
        // lua.context(|lua_ctx| {
        //     let print_order = lua_ctx.create_function(|_, s: String| {
        //         println!("order: {}", s);
        //         Ok(())
        //     }).unwrap();

        //     let get_setting = lua_ctx.create_function(|_, setting_key: String| {
        //         let setting_value = get_setting_value(&mut settings, &setting_key);
        //         Ok(setting_value)
        //     }).unwrap();

        //     lua_ctx.globals().set("print_order", print_order).unwrap();
        //     lua_ctx.globals().set("get_setting", get_setting).unwrap();

        //     lua_ctx.load(&strategy.lua_script).exec().unwrap();
        // });

        // load the lua stript for the strategy
        lua.context(|lua_context| {

            let print_order = lua_context.create_function(|_, s: String| {
                println!("order: {}", s);
                Ok(())
            }).unwrap();

            let get_setting = lua_context.create_function(|_, key: String| {
                let value = &strategy.settings.get(&key).unwrap();
                Ok(value)
            }).unwrap();

            // let get_setting = lua_context.create_function(|_, key: String| {
            //     let value = strategy.settings.get(&key).unwrap();
            //     Ok(value)
            // }).unwrap();

            lua_context.globals().set("print_order", print_order).unwrap();

            lua_context.load(&strategy.lua_script).exec().unwrap();
        });
    }

    // println!("datasets loaded from: {}", config.datasets_path);

    // Read data csv file
    // let data = utils::read_data(config.datasets_path);

    // Store rolling window of the last 200 rows of data
    // let mut window = Vec::new();

    // Loop through each row of data
    for row in data {
        // Add the current row to the window
        window.push(row);

        // If the window is full, remove the oldest row
        if window.len() > 200 {
            window.remove(0);
        }

        // Run the strategy
        // strategy.run(window.clone());

        // Print the current row
        println!("{:?}", window.last().unwrap());
    }
    
}*/
