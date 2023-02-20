use std::sync::mpsc::{channel, Sender};
use std::thread;
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn main() {
    // create a channel to communicate between threads
    let (tx, rx) = channel();

    let mut rl = Editor::<()>::new().unwrap();

    let mut input_buffer = String::new();

    loop {
        let readline = rl.readline("backtester> ");

        if let Err(_) = readline {
            // send the exit command to the main thread
            tx.send("exit").unwrap();
            break;
        }

        input_buffer = readline.unwrap();

        let command = input_buffer.as_str();

        if command == "exit" || command == "quit" {
            // send the exit command to the main thread
            tx.send("exit").unwrap();
            break;
        }

        // let line = String::from(line).as_str();
        // add the line to the history and send it to the main thread
        rl.add_history_entry(command);
        tx.send(command).unwrap();
    }

    // create a thread to handle user input
    /*let input_thread = thread::spawn(move || {
        // create a new line editor
        let mut rl = Editor::<()>::new().unwrap();
        
        loop {
            let readline = rl.readline("> ");

            if let Err(_) = readline {
                // send the exit command to the main thread
                tx.send("exit").unwrap();
                break;
            }

            let command = readline.unwrap().clone().as_str();

            if command == "exit" || command == "quit" {
                // send the exit command to the main thread
                tx.send("exit").unwrap();
                break;
            }

            // let line = String::from(line).as_str();
            // add the line to the history and send it to the main thread
            rl.add_history_entry(command);
            tx.send(command).unwrap();

            /*match readline {
                Ok(line) => {
                    // let command = line.clone();
                    if command == "exit" || command == "quit" {
                        // send the exit command to the main thread
                        tx.send("exit").unwrap();
                        break;
                    }
                    else {
                        // let line = String::from(line).as_str();
                        // add the line to the history and send it to the main thread
                        rl.add_history_entry(&command);
                        tx.send(command.clone()).unwrap();
                    }
                },
                Err(_) => {
                    // send the exit command to the main thread
                    tx.send("exit").unwrap();
                    break;
                },
            }*/
        }
    });*/

    let mut threads: Vec<(String, Sender<&str>)> = vec![];

    loop {
        if let Ok(message) = rx.recv() {
            let words: Vec<&str> = message.split_whitespace().collect();
            match words.first() {
                Some(&"exit") => break,
                Some(&"help") => {
                    println!("Available commands: help, exit, start [thread name], stop [thread name], pause [thread name], resume [thread name]");
                },
                Some(&"start") => {
                    if words.len() != 2 {
                        println!("Usage: start [thread name]");
                    }
                    else {
                        let name = words[1].to_string();
                        let new_thread_name = name.clone();
                        let (tx, rx) = channel();

                        let strategy_thread = thread::spawn(move || {
                            let mut count = 0;
                            loop {
                                if count == 100 {
                                    println!("Thread {} finished counting", new_thread_name);
                                    break;
                                }

                                match rx.try_recv() {
                                    Ok(command) => {
                                        match command {
                                            "pause" => {
                                                println!("Paused {}", new_thread_name);
                                            },
                                            "resume" => {
                                                println!("Resumed {}", new_thread_name);
                                            },
                                            "stop" | "exit" => {
                                                println!("Stopped {}", new_thread_name);
                                                break;
                                            },
                                            _ => {
                                                println!("Invalid command");
                                            }
                                        }
                                    },
                                    _ => {
                                        count += 1;
                                        println!("Thread {} count: {}", new_thread_name, count);
                                        thread::sleep(std::time::Duration::from_secs(1));
                                    },
                                }

                                // println!("Thread {} count: {}", new_thread_name, count);
                                count += 1;
                                thread::sleep(std::time::Duration::from_secs(1));
                            }
                        });

                        strategy_thread.join().unwrap();

                        threads.push((name.to_string(), tx));
                        println!("Thread {} started", name.to_string());
                    }
                }
                Some(&"stop") => {
                    if words.len() != 2 {
                        println!("Usage: stop [thread name]");
                    }
                    else {
                        let name = words[1].to_string();
                        if let Some((_, tx)) = threads.iter().find(|(n, _)| *n == name) {
                            tx.send("stop").unwrap();
                            threads.retain(|(n, _)| *n != name);
                            println!("Stoppping {}", name);
                        }
                        else {
                            println!("Thread {} not found", name);
                        }
                    }
                },
                Some(&"pause") => {
                    if words.len() != 2 {
                        println!("Usage: pause [thread name]");
                    }
                    else {
                        let name = words[1].to_string();
                        if let Some((_, tx)) = threads.iter().find(|(n, _)| *n == name) {
                            tx.send("pause").unwrap();
                            println!("Pausing {}", name);
                        }
                        else {
                            println!("Thread {} not found", name);
                        }
                    }
                },
                Some(&"resume") => {
                    if words.len() != 2 {
                        println!("Usage: resume [thread name]");
                    }
                    else {
                        let name = words[1].to_string();
                        if let Some((_, tx)) = threads.iter().find(|(n, _)| *n == name) {
                            tx.send("resume").unwrap();
                            println!("Resuming {}", name);
                        }
                        else {
                            println!("Thread {} not found", name);
                        }
                    }
                },
                _ => {
                    println!("Invalid command. Type 'help' for available commands");
                },
            }
        }
    }

    for thread in threads {

    }

    // input_thread.join().unwrap();
}
