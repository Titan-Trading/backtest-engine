use std::{collections::HashMap, thread::JoinHandle};

use crate::{utils::Config, datasets::{Dataset, load_datasets}, plugins::{strategies::{Strategy, load_strategy_plugins, StrategyPlugin}, indicators::{load_indicators, IndicatorPlugin}}, threads::{ThreadManager}};

pub struct Core {
    pub config: Config,
    pub thread_manager: ThreadManager,
    pub datasets: HashMap<String, Dataset>,
    pub indicators: HashMap<String, IndicatorPlugin>,
    pub strategies: HashMap<String, StrategyPlugin>,
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
    fn set_plugins(&mut self, indicators: HashMap<String, IndicatorPlugin>, strategies: HashMap<String, StrategyPlugin>) {
        self.indicators = indicators;
        self.strategies = strategies;
    }

    // initialize/reinitialize the system by reloading all the plugins
    pub fn initialize(&mut self) {
        // Load datasets
        let datasets = load_datasets(self.config.datasets_path.to_owned()).unwrap();

        // Load indicators
        let indicators = load_indicators(self.config.indicators_path.to_owned()).unwrap();

        // Load strategies
        let strategies = load_strategy_plugins(self.config.strategies_path.to_owned()).unwrap();

        self.set_datasets(datasets);
        self.set_plugins(indicators, strategies);
    }

    // start a strategy instance (thread)
    pub fn start(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        // get strategy plugin from list of load plugins
        let strategy_plugin = self.strategies.get(&strategy_name.clone()).unwrap();
        let strategy_settings = strategy_plugin.settings.clone();
        let strategy_script = strategy_plugin.lua_script.clone();

        // use strategy plugin's settings to put together a list of datasets
        let mut datasets = HashMap::new();
        let datasets_string = strategy_settings.get("datasets").unwrap();
        let datasets_parts: Vec<&str> = datasets_string.split(",").collect();
        for dataset_name in datasets_parts {
            let name = dataset_name.to_string();
            let dataset = Dataset::new(name.clone());
            datasets.insert(name, dataset);
        }

        // create and initialize strategy
        let strategy = Strategy::new(strategy_name.clone(), strategy_script, strategy_settings, datasets);

        /*let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "stopped" {
            let is_started = self.thread_manager.start(strategy_name.clone(), move || {
                // println!("Inside strategy thread, implement LUA and loop through datasets");

                // load up LUA script
                // setup synchronized datasets stream
                // start loop through each iteration
                // track orders
                // output orders
                // output performance metrics
                // strategy.run();

            });

            return is_started;
        }*/

        false
    }

    // stop a strategy instance (thread)
    pub fn stop(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "running" {
            return self.thread_manager.stop(strategy_name.clone());
        }

        false
    }

    // pause a strategy instance (thread)
    pub fn pause(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "running" {
            return self.thread_manager.pause(strategy_name.clone());
        }

        false
    }

    // resume a strategy instance (thread)
    pub fn resume(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "stopped" {
            return self.thread_manager.resume(strategy_name.clone());
        }

        false
    }

    // get the status of a strategy instance (thread)
    pub fn status(&mut self, strategy_name: String) -> String {
        // check if we have the strategy loaded
        if !self.strategies.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return "not found".to_string();
        }

        let state = self.thread_manager.get_state(strategy_name.clone());

        return state;
    }
}