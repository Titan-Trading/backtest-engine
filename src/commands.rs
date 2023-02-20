use crate::{system::Core};



pub enum CommandType {
    // general commands
    Help,
    Exit,
    ReloadPlugins, // reload all datasets, strategies, indicators, etc.

    // data commands
    ListDatasets,
    ListStrategies,
    ListIndicators,

    // strategy commands (controlling running strategies)
    StatusStrategyThread,
    StartStrategyThread,
    StopStrategyThread,
    PauseStrategyThread,
    ResumeStrategyThread,

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

        // data commands
        else if raw_input.starts_with("list") {
            let params: Vec<&str> = raw_input.split_whitespace().collect();

            if params.len() == 2 {
                let list_type = params[1];

                if list_type == "datasets" || list_type == "dataset" {
                    command_type = CommandType::ListDatasets;
                }
                else if list_type == "strategies" || list_type == "strategy" {
                    command_type = CommandType::ListStrategies;
                }
                else if list_type == "indicators" || list_type == "indicator" {
                    command_type = CommandType::ListIndicators;
                }   
            }
        }

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
        CommandType::Help => {
            // output help messaging
            println!("Available commands:");
            println!("list   [type]          - lists out the loaded items of a given type (datasets, strategies, indicators, etc.)");
            println!("start  [strategy name] - starts a new strategy thread");
            println!("stop   [strategy name] - stops the strategy thread");
            println!("pause  [strategy name] - pauses the strategy thread");
            println!("resume [strategy name] - resumes a paused strategy thread");
            println!("reload                 - reloads all the plugins (datasets, strategies, indicators, etc.)");
            println!("help                   - displays this help message");
            println!("exit                   - exits the program");
            // continue the loop
        },
        CommandType::Exit => {
            // break outside the loop
            return true;
        },
        CommandType::ReloadPlugins => {
            core.initialize();

            println!("plugins reloaded");
        },
        CommandType::ListDatasets => {
            for (key, dataset) in core.datasets.iter() {
                println!("{}: {:?}", key, dataset.files);
            }
        },
        CommandType::ListStrategies => {
            for (key, strategy) in core.strategies.iter() {
                println!("{}: {:?}", key, strategy.settings);
            }
        },
        CommandType::ListIndicators => {
            for (key, _indicator) in core.indicators.iter() {
                println!("{}", key);
            }
        },
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
        CommandType::Unsupported => {
            println!("Unsupported command");
        }
    }

    return false;
}