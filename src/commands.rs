use crate::{system::Core, datasets::data_provider::DataProviderRepository};


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
    SearchData,   // search for a data provider using a keyword like exchange and symbol
    DownloadData, // download data from a data provider

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

        // parse params
        let params: Vec<&str> = raw_input.split_whitespace().collect();

        //////////////////////
        // general commands //
        //////////////////////
        if raw_input == "help" {
            command_type = CommandType::Help;
        }
        else if raw_input == "quit" || raw_input == "exit" {
            command_type = CommandType::Exit;
        }
        else if raw_input == "reload" {
            command_type = CommandType::ReloadPlugins;
        }

        ///////////////////
        // list commands //
        ///////////////////
        else if raw_input.starts_with("list") {
            // list types: datasets, indicators, strategies
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

        ///////////////////
        // data commands //
        ///////////////////
        else if raw_input.starts_with("search") {
            if params.len() == 3 {
                let exchange = params[1]; // ex: binance
                let symbol = params[2];   // ex: BTCUSDT

                command_type = CommandType::SearchData;
                command_params.push(String::from(exchange));
                command_params.push(String::from(symbol));
            }
        }
        else if raw_input.starts_with("download") {
            if params.len() == 2 {
                let data_source_id = params[1];

                command_type = CommandType::DownloadData;
                command_params.push(String::from(data_source_id));
            }
        }

        ///////////////////////
        // strategy commands //
        ///////////////////////
        else if 
            raw_input.starts_with("status")
            || raw_input.starts_with("start")
            || raw_input.starts_with("stop")
            || raw_input.starts_with("pause")
            || raw_input.starts_with("resume") {
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

    let mut data_providers = DataProviderRepository::new();

    // perform different actions based on command type
    match command.command_type {
        // output help messaging
        CommandType::Help => {
            println!("available commands");
            println!("list     [type] - lists out the loaded items of a given type (datasets, strategies, indicators, etc.)");
            println!("search   [exchange] [symbol] - searches all supported data providers by a given exchange and symbol");
            println!("download [provider id] [exchange] [symbol] - downloads data for a given provider, exchange and symbol from a data provider");
            println!("start    [strategy name] - starts a new strategy thread");
            println!("stop     [strategy name] - stops the strategy thread");
            println!("pause    [strategy name] - pauses the strategy thread");
            println!("resume   [strategy name] - resumes a paused strategy thread");
            println!("reload - reloads all the plugins (datasets, strategies, indicators, etc.)");
            println!("help   - displays this help message");
            println!("exit   - exits the program");
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
            let start_timestamp = command.params.get(2).unwrap();
            let end_timestamp = command.params.get(3).unwrap();

            let result = data_providers.search(
                exchange.clone(),
                symbol.clone(),
                start_timestamp.clone().parse::<i64>().unwrap(),
                Some(end_timestamp.clone().parse::<i64>().unwrap())
            );
        },
        CommandType::DownloadData => {
            let data_source_id = command.params.get(0).unwrap();

            let result = data_providers.download(data_source_id.clone().parse::<i32>().unwrap());

            println!("Downloading data for {}", data_source_id);
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