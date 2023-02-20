
use rlua::{Result, Lua};
use rustyline::{self, error::ReadlineError};
use system::Core;
use crate::repositories::datasets::{load_datasets};
use crate::repositories::strategies::{load_strategies};

mod system;
mod utils;
mod repositories;

fn initialize_system(core: &mut Core) -> () {

    // Load datasets
    // - get list of all folder in datasets folder
    // - for each folder, load the data.csv file into a Vec<Vec<f64>>
    // - store the Vec<Vec<f64>> in a HashMap<String, Vec<Vec<f64>>>
    let datasets = load_datasets(core.config.datasets_path.to_owned()).unwrap();

    // Load strategies
    // - get list of all folders in strategies folder
    // - for each folder, load the settings.txt file into a HashMap<String, String>
    // - store the HashMap<String, String> in a HashMap<String, HashMap<String, String>>
    // - for each folder, load the strategy.lua file into a String
    // - store the String in a HashMap<String, String>
    let strategies = load_strategies(core.config.strategies_path.to_owned()).unwrap();

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

        
    }

    core.set_datasets(datasets.clone());
    core.set_strategies(strategies.clone());

    ()
}

fn main() {
    // Get the path to the config file from the command line
    let config_path = std::env::args().nth(1).unwrap_or("config.txt".to_string());

    // Read the config file into a Config struct
    let config = utils::read_config(config_path);

    // Create the system core
    let mut core = Core::new(&config);

    // Initialize the system the first time
    initialize_system(&mut core);

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    // Print the welcome message
    println!("------------------------------------------------------------------");
    println!("Welcome to the backtester (type 'exit' to quit or 'help' for help)");
    println!("------------------------------------------------------------------");

    // REPL loop
    // let mut input_buffer = String::new();

    loop {
        let input_buffer: String<'static> = match rl.readline("> ") {
            Ok(line) => line.clone(),
    
            // Exit on CTRL-C or CTRL-D
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };

        // General commands
        if input_buffer == "exit" || input_buffer == "quit" || input_buffer == "q" {
            println!("exiting...");
            break;
        }
        else if input_buffer == "help" ||  input_buffer == "h" {
            println!("")
        }

        // System commands
        else if input_buffer == "list strategies" {
            for (index, strategy) in core.strategies.iter().enumerate() {
                println!("{}: {}", index, strategy.name);
            }
        }
        else if input_buffer == "list datasets" {
            for (index, dataset) in core.datasets.iter().enumerate() {
                println!("{}: {}", index, dataset.name);
            }
        }
        else if input_buffer == "reload" {
            initialize_system(&mut core);

            println!("plugins reloaded");
        }

        // Strategy commands
        else if input_buffer.starts_with("start") {
            // check if strategy exists
            let words: Vec<_> = input_buffer.split_whitespace().collect();
            let strategy_name = words.get(1).unwrap().clone();
            let mut found = false;

            for strategy in &core.strategies {
                if strategy.name == strategy_name {
                    found = true;
                    break;
                }
            }

            if !found {
                println!("could not find strategy: {}", strategy_name);
                continue;
            }

            // check if strategy is already running
            if core.strategy_threads.contains_key(&strategy_name.to_string()) {
                println!("strategy is already running: {}", strategy_name);
                continue;
            }

            // start the strategy in a new thread
            let datasets = core.datasets.clone();
            let strategies = core.strategies.clone();

            let strategy_thread = std::thread::Builder::new().name(strategy_name.to_string()).spawn(move || {
                let words: Vec<_> = input_buffer.split_whitespace().collect();
                let strategy = strategies.iter().find(|s| s.name == strategy_name).unwrap().clone();

                // load the lua script for the strategy
                let lua = rlua::Lua::new();
                lua.context(|lua_context| {
                    let print_order = lua_context.create_function(|_, s: String| {
                        println!("order: {}", s);
                        Ok(())
                    }).unwrap();

                    lua_context.globals().set("print_order", print_order).unwrap();

                    lua_context.load(&strategy.lua_script).exec().unwrap();
                });

                // run the strategy
                strategy.run(datasets);
            }).unwrap();

            // join the strategy thread so it's connected to the main thread
            strategy_thread.join().unwrap();

            // add the strategy thread to the list of running threads
            core.strategy_threads.insert(strategy_name.to_string(), strategy_thread);

            println!("strategy started: {}", strategy_name);
        }
        else if input_buffer.starts_with("stop") {
            println!("stop strategy");
        }
        else if input_buffer.starts_with("pause") {
            println!("pause strategy");
        }
        else if input_buffer.starts_with("resume") {
            println!("resume strategy");
        }

        // Test commands
        else if input_buffer.starts_with("test") {
            println!("test strategy or indicator");
        }

        // Unknown command
        else {
            println!("unknown command: {}", input_buffer);
        }
    }
}