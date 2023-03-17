use crate::{system::Core, datasets::data_provider::DataProvider};



pub enum CommandType {
    // general commands
    Help,          // print help message
    Exit,          // exit the program
    ReloadPlugins, // reload all datasets, strategies, indicators, etc.

    // list commands
    ListDatasets,   // comes from the database index
    ListStrategies, // comes from the strategies directory
    ListIndicators, // comes from the indicators directory

    // data download commands
    SearchData, // search for a data provider using a keyword like exchange and symbol
    DownloadData,       // download data from a data provider

    // strategy commands (controlling strategy threads)
    StatusStrategyThread, // get the status of a strategy thread
    StartStrategyThread,  // start a new strategy thread
    StopStrategyThread,   // stop a running strategy thread
    PauseStrategyThread,  // pause a running strategy thread
    ResumeStrategyThread, // resume a paused strategy thread

    // unsupported command
    Unsupported,
}


pub struct Command {
    pub raw: String,
    pub command_type: CommandType,
    pub params: Vec<String>
}

impl Command {
    pub fn new(raw_input: String) -> Command {
        let mut command_type: CommandType = CommandType::Unsupported;
        let mut command_params: Vec<String> = Vec::new();

        // general commands
        if raw_input == "help" {
            command_type = CommandType::Help;
        }
        else if raw_input == "quit" || raw_input == "exit" {
            command_type = CommandType::Exit;
        }
        else if raw_input == "reload" {
            command_type = CommandType::ReloadPlugins;
        }

        // list commands
        else if raw_input.starts_with("list") {
            let params: Vec<&str> = raw_input.split_whitespace().collect();

            if params.len() == 2 {
                let list_type = params[1];

                if list_type == "datasets" || list_type == "dataset" {
                    command_type = CommandType::ListDatasets;
                }
                else if list_type == "indicators" || list_type == "indicator" {
                    command_type = CommandType::ListIndicators;
                }
                else if list_type == "strategies" || list_type == "strategy" {
                    command_type = CommandType::ListStrategies;
                }
            }
        }

        // data commands
        else if raw_input.starts_with("search") {
            let params: Vec<&str> = raw_input.split_whitespace().collect();

            if params.len() == 3 {
                let exchange = params[1];
                let symbol = params[2];

                command_type = CommandType::SearchData;
                command_params.push(String::from(exchange));
                command_params.push(String::from(symbol));
            }
        }
        else if raw_input.starts_with("download") {
            let params: Vec<&str> = raw_input.split_whitespace().collect();

            if params.len() == 3 {
                let exchange = params[1];
                let symbol = params[2];

                command_type = CommandType::DownloadData;
                command_params.push(String::from(exchange));
                command_params.push(String::from(symbol));
            }
        }

        // strategy commands
        else if raw_input.starts_with("status") || raw_input.starts_with("start") || raw_input.starts_with("stop") || raw_input.starts_with("pause") || raw_input.starts_with("resume") {
            let params: Vec<&str> = raw_input.split_whitespace().collect();

            if params.len() == 2 {
                let command_action = String::from(params[0]);
                let thread_name = String::from(params[1]);

                command_params.push(thread_name);

                if command_action == "status" {
                    command_type = CommandType::StatusStrategyThread;
                }
                else if command_action == "start" {
                    command_type = CommandType::StartStrategyThread;
                }
                else if command_action == "stop" {
                    command_type = CommandType::StopStrategyThread;
                }
                else if command_action == "pause" {
                    command_type = CommandType::PauseStrategyThread;
                }
                else if command_action == "resume" {
                    command_type = CommandType::ResumeStrategyThread;
                }
            }
        }

        Command {
            raw: raw_input,
            command_type: command_type,
            params: command_params
        }
    }
}

// handle commands coming from REPL input
pub fn handle_command(core: &mut Core, command: Command) -> bool {
    // perform different actions based on command type
    match command.command_type {
        // output help messaging
        CommandType::Help => {
            println!("available commands:");
            println!("list     [type]              - lists out the loaded items of a given type (datasets, strategies, indicators, etc.)");
            println!("search   [exchange] [symbol] - searches all supported data providers by a given exchange and symbol");
            println!("download [exchange] [symbol] - downloads data for a given exchange and symbol from a data provider");
            println!("start    [strategy name]     - starts a new strategy thread");
            println!("stop     [strategy name]     - stops the strategy thread");
            println!("pause    [strategy name]     - pauses the strategy thread");
            println!("resume   [strategy name]     - resumes a paused strategy thread");
            println!("reload                       - reloads all the plugins (datasets, strategies, indicators, etc.)");
            println!("help                         - displays this help message");
            println!("exit                         - exits the program");
            // continue the loop
        },

        // exit the program
        CommandType::Exit => {
            // break outside the loop
            return true;
        },

        // reload plugins
        CommandType::ReloadPlugins => {
            core.initialize();

            println!("plugins reloaded");
        },

        // list commands
        CommandType::ListStrategies => {
            for (key, strategy) in core.strategy_plugins.iter() {
                println!("{}: {:?}", key, strategy.settings);
            }
        },
        CommandType::ListDatasets => {
            for (key, dataset) in core.available_datasets.iter() {
                println!("{}: {:?}", key, dataset);
            }
        },
        CommandType::ListIndicators => {
            for (key, _indicator) in core.indicator_plugins.iter() {
                println!("{}", key);
            }
        },

        // data commands
        CommandType::SearchData => {
            let exchange = command.params.get(0).unwrap();
            let symbol = command.params.get(1).unwrap();

            let mut data_providers = Vec::new();
            data_providers.push(DataProvider::new(
                "Yahoo Finance".to_string(), 
                "https://finance.yahoo.com/quote/{}.{}/history?p={}.{}".to_string()
            ));
            data_providers.push(DataProvider::new(
                "Google Finance".to_string(), 
                "https://www.google.com/finance/historical?q={}.{}&startdate=Jan+1%2C+2010&enddate=Dec+31%2C+2018&output=csv".to_string()
            ));
            data_providers.push(DataProvider::new(
                "CryptoCompare".to_string(), 
                "https://min-api.cryptocompare.com/data/histoday?fsym={}&tsym=USD&limit=2000&aggregate=1&e=CCCAGG".to_string()
            ));
            data_providers.push(DataProvider::new(
                "CryptoDataDownload".to_string(), 
                "https://www.cryptodatadownload.com/cdd/Bitstamp_{}_USD_1h.csv".to_string()
            ));

            for provider in data_providers {
                println!("Searching for {} - {} on {}", symbol, exchange, provider.name);
            }

            // let data_providers = core.search_data_provider(search_term.to_string());

            // for data_provider in data_providers {
            //     println!("{}: {}", data_provider.name, data_provider.description);
            // }
        },
        CommandType::DownloadData => {
            let exchange = command.params.get(0).unwrap();
            let symbol = command.params.get(1).unwrap();

            println!("Downloading data for {} - {}", exchange, symbol);
        },

        // strategy commands
        CommandType::StatusStrategyThread => {
            let strategy_name = command.params.get(0).unwrap();

            let status = core.status(strategy_name.to_string());
            
            println!("Strategy thread is {}", status);
        },
        CommandType::StartStrategyThread => {
            println!("Start strategy thread");

            let strategy_name = command.params.get(0).unwrap();

            let is_started: bool = core.start(strategy_name.to_string());
            if is_started {
                println!("Strategy thread has been started");
            }
            else {
                println!("Strategy thread was unable to be started");
            }
        },
        CommandType::StopStrategyThread => {
            println!("Stop strategy thread");

            let strategy_name = command.params.get(0).unwrap();

            let is_stopped: bool = core.stop(strategy_name.to_string());
            if is_stopped {
                println!("Strategy thread has been stopped");
            }
            else {
                println!("Strategy thread was unable to be stopped");
            }
        },
        CommandType::PauseStrategyThread => {
            println!("Pause strategy thread");

            let strategy_name = command.params.get(0).unwrap();

            let is_paused: bool = core.pause(strategy_name.to_string());
            if is_paused {
                println!("Strategy thread has been paused");
            }
            else {
                println!("Strategy thread was unable to be paused");
            }
        },
        CommandType::ResumeStrategyThread => {
            println!("Resume strategy thread");

            let strategy_name = command.params.get(0).unwrap();

            let is_resumed: bool = core.resume(strategy_name.to_string());
            if is_resumed {
                println!("Strategy thread has been started");
            }
            else {
                println!("Strategy thread was unable to be started");
            }
        },

        // command not supported
        CommandType::Unsupported => {
            println!("Unsupported command");
        }
    }

    return false;
}