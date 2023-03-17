use std::{collections::HashMap, thread::JoinHandle};
use mlua::Lua;

use crate::{utils::Config, threads::ThreadManager, plugins::{indicators::{indicator_plugin::{IndicatorPlugin, load_indicator_plugins}}, strategies::{strategy_plugin::{StrategyPlugin, load_strategy_plugins}, strategy::Strategy, strategy_manager::StrategyManager}}, datasets::dataset::{load_datasets, Dataset}, database::{database::Database, models::index::Corpus}};


pub struct Core {
    pub config: Config,
    pub database: Database,
    pub available_datasets: HashMap<String, Corpus>,
    pub thread_manager: ThreadManager,
    pub indicator_plugins: HashMap<String, IndicatorPlugin>,
    pub strategy_plugins: HashMap<String, StrategyPlugin>,
    pub strategies: StrategyManager,
    pub running_strategies: HashMap<String, JoinHandle<()>>,
}

impl Core {
    // create a new system core with a given config
    pub fn new(config: &Config) -> Core {
        Core {
            config: config.clone(),
            database: Database::new(),
            available_datasets: HashMap::new(),
            thread_manager: ThreadManager::new(),
            indicator_plugins: HashMap::new(),
            strategy_plugins: HashMap::new(),
            strategies: StrategyManager::new(),
            running_strategies: HashMap::new(),
        }
    }

    // set the datasets that have been loaded
    pub fn set_datasets(&mut self, datasets: HashMap<String, Corpus>) {
        self.available_datasets = datasets;
    }

    // set the indicators that have been loaded
    fn set_plugins(&mut self, indicators: HashMap<String, IndicatorPlugin>, strategies: HashMap<String, StrategyPlugin>) {
        self.indicator_plugins = indicators;
        self.strategy_plugins = strategies;
    }

    // initialize/reinitialize the system by reloading all the plugins
    pub fn initialize(&mut self) {
        // load list of a datasets from database engine index
        let datasets = load_datasets(&mut self.database).unwrap();
        // let datasets = HashMap::new();

        // load plugins
        let indicators = load_indicator_plugins(self.config.indicators_path.to_owned()).unwrap();
        let strategies  = load_strategy_plugins(self.config.strategies_path.to_owned()).unwrap();

        // set datasets and plugins
        self.set_datasets(datasets);
        self.set_plugins(indicators, strategies);
    }

    // start a strategy instance (thread)
    pub fn start(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategy_plugins.contains_key(&strategy_name) {
            println!("Strategy not found");
            return false;
        }

        // get strategy plugin from list of load plugins
        let strategy_plugin = self.strategy_plugins.get(&strategy_name).unwrap();
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

        let strategies = self.strategies.clone();

        // check if the strategy thread is stopped
        // if it is not, return false
        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "stopped" {

            let lua = Lua::new();

            let is_started = self.thread_manager.start(strategy_name.clone(), move || {
                println!("inside strategy thread, implement LUA and loop through datasets");

                // check if strategy is already running
                // if strategies.get(&strategy_name).is_err() {
                //     println!("Strategy already running");
                    
                //     // create and initialize strategy
                //     let strategy = Strategy::new(
                //         strategy_name.clone(),
                //         strategy_script.clone(),
                //         strategy_settings.clone(),
                //         datasets.clone()
                //     );
                // }

                // load up LUA script
                // setup synchronized datasets stream
                // start loop through each iteration
                // track orders
                // output orders
                // output performance metrics
                // strategy.run();

            });

            return is_started;
        }

        false
    }

    // stop a strategy instance (thread)
    pub fn stop(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategy_plugins.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        // check if the strategy thread is running
        // if it is, stop it
        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "running" {
            return self.thread_manager.stop(strategy_name.clone());
        }

        false
    }

    // pause a strategy instance (thread)
    pub fn pause(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategy_plugins.contains_key(&strategy_name.clone()) {
            println!("Strategy not found");
            return false;
        }

        // check if the strategy thread is running
        // if it is, pause it
        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "running" {
            return self.thread_manager.pause(strategy_name.clone());
        }

        false
    }

    // resume a strategy instance (thread)
    pub fn resume(&mut self, strategy_name: String) -> bool {
        // check if we have the strategy loaded
        if !self.strategy_plugins.contains_key(&strategy_name) {
            println!("Strategy not found");
            return false;
        }

        // check if the strategy thread is paused
        // if it is, resume it
        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "paused" {
            return self.thread_manager.resume(strategy_name.clone());
        }

        false
    }

    // get the status of a strategy instance (thread)
    pub fn status(&mut self, strategy_name: String) -> String {
        // check if we have the strategy loaded
        if !self.strategy_plugins.contains_key(&strategy_name) {
            println!("Strategy not found");
            return "not found".to_string();
        }

        // get the state of the thread
        return self.thread_manager.get_state(strategy_name);
    }
}