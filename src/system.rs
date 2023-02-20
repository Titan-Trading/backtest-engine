use std::{collections::HashMap, thread::JoinHandle};

use crate::{utils::Config, plugins::{strategies::{Strategy, load_strategies}, datasets::{Dataset, load_datasets}, indicators::{load_indicators, Indicator}}, threads::{ThreadManager}};

pub struct Core {
    pub config: Config,
    pub thread_manager: ThreadManager,
    pub datasets: HashMap<String, Dataset>,
    pub indicators: HashMap<String, Indicator>,
    pub strategies: HashMap<String, Strategy>,
    pub running_strategies: HashMap<String, JoinHandle<()>>,
}

impl Core {
    // create a new system core with a given config
    pub fn new(config: &Config) -> Core {
        Core {
            config: config.clone(),
            thread_manager: ThreadManager::new(),
            datasets: HashMap::new(),
            indicators: HashMap::new(),
            strategies: HashMap::new(),
            running_strategies: HashMap::new(),
        }
    }

    // set the datasets that have been loaded
    pub fn set_datasets(&mut self, datasets: HashMap<String, Dataset>) {
        self.datasets = datasets;
    }

    // set the indicators that have been loaded
    pub fn set_indicators(&mut self, indicators: HashMap<String, Indicator>) {
        self.indicators = indicators;
    }

    // set the strategies that have been loaded
    pub fn set_strategies(&mut self, strategies: HashMap<String, Strategy>) {
        self.strategies = strategies;
    }

    // initialize/reinitialize the system by reloading all the plugins
    pub fn initialize(&mut self) {
        // Load datasets
        // - get list of all folder in datasets folder
        // - for each folder, load the data.csv file into a Vec<Vec<f64>>
        // - store the Vec<Vec<f64>> in a HashMap<String, Vec<Vec<f64>>>
        let datasets = load_datasets(self.config.datasets_path.to_owned()).unwrap();

        // Load indicators
        // - get list of all files in indicators folder
        // - load each lua file into a string
        // - store lua script for later
        let indicators = load_indicators(self.config.indicators_path.to_owned()).unwrap();

        // Load strategies
        // - get list of all folders in strategies folder
        // - for each folder, load the settings.txt file into a HashMap<String, String>
        // - store the HashMap<String, String> in a HashMap<String, HashMap<String, String>>
        // - for each folder, load the strategy.lua file into a String
        // - store the String in a HashMap<String, String>
        let strategies = load_strategies(self.config.strategies_path.to_owned()).unwrap();

        self.set_datasets(datasets);
        self.set_indicators(indicators);
        self.set_strategies(strategies);
    }

    // start a strategy instance (thread)
    pub fn start(&mut self, thread_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&thread_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(thread_name.clone());
        if state == "stopped" {
            let is_started = self.thread_manager.start(thread_name.clone(), move || {
                // println!("Inside strategy thread, implement LUA and loop through datasets");

                // load up LUA script
                // setup synchronized datasets stream
                // start loop through each iteration
                // track orders
                // output orders
                // output performance metrics

            });

            return is_started;
        }

        false
    }

    // stop a strategy instance (thread)
    pub fn stop(&mut self, thread_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&thread_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(thread_name.clone());
        if state == "running" {
            return self.thread_manager.stop(thread_name.clone());
        }

        false
    }

    // pause a strategy instance (thread)
    pub fn pause(&mut self, thread_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&thread_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(thread_name.clone());
        if state == "running" {
            return self.thread_manager.pause(thread_name.clone());
        }

        false
    }

    // resume a strategy instance (thread)
    pub fn resume(&mut self, thread_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&thread_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(thread_name.clone());
        if state == "stopped" {
            return self.thread_manager.resume(thread_name.clone());
        }

        false
    }

    // get the status of a strategy instance (thread)
    pub fn status(&mut self, thread_name: String) -> String {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&thread_name.clone()) {
            println!("Strategy not found");
            return "not found".to_string();
        }

        let state = self.thread_manager.get_state(thread_name.clone());

        return state;
    }
}