use commands::{Command, handle_command};
use rustyline::Editor;

// bring modules into scope
mod utils;
mod models;
mod commands;
mod threads;
mod datasets;
mod plugins;
mod system;

fn main() {
    // Get the path to the config file from the command line
    let config_path = std::env::args().nth(1).unwrap_or("config.txt".to_string());

    // Read the config file into a Config struct
    let config = utils::read_config(config_path);

    // create a REPL editor using rustyline
    let mut rl = Editor::<()>::new().unwrap();

    // load REPL history
    if rl.load_history("./.engine-history.txt").is_err() {
        println!("No history found");
    }

    // Print the welcome message
    println!("-----------------------------------------------------------------");
    println!("Trade Engine version 0.1 (type 'exit' to quit or 'help' for help)");
    println!("-----------------------------------------------------------------");

    // create a core system
    let mut core = system::Core::new(&config.clone());

    // initialize the system by load plugins
    core.initialize();

    // update loop
    loop {
        // output a prompt and wait for input
        let input = rl.readline("engine> ");

        // handle the result
        match input {
            Ok(line) => {
                let raw_line = line;
                // add the input to our REPL history
                rl.add_history_entry(raw_line.clone());

                // parse the command
                let command = Command::new(raw_line.clone());

                // handle REPL commands and exit if we need to
                let should_exit = handle_command(&mut core, command);
                if should_exit {
                    break;
                }
            },
            Err(_) => {
                break;
            }
        }
    }

    // save our REPL history to a file
    rl.save_history("./.engine-history.txt").unwrap();
}