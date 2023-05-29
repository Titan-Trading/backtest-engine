use std::{collections::HashMap, thread::JoinHandle, cell::RefCell};

use crate::{utils::config::Config, threads::ThreadManager, plugins::{indicators::{indicator_plugin::{IndicatorPlugin, load_indicator_plugins}}, strategies::{strategy_plugin::{StrategyPlugin, load_strategy_plugins}, strategy::Strategy, strategy_manager::StrategyManager}}, datasets::dataset::load_datasets, database::{database::Database, models::index::Corpus}};


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

        let strategies = RefCell::new(self.strategies.clone());

        // check if the strategy thread is stopped
        // if it is stopped start a new thread
        let state = self.thread_manager.get_state(strategy_name.clone());
        if state == "stopped" {
            // start thread
            println!("Starting strategy thread");
            let is_started = self.thread_manager.start(strategy_name.clone(), move || {
                let mut strategies = strategies.borrow_mut();

                // get strategy from list of running strategies
                let strategy_result = strategies.get(&strategy_name);
                
                // if strategy is not running, create and initialize it
                let mut strategy = match strategy_result {
                    Ok(strategy) => strategy.clone(),
                    Err(_) => {
                        println!("Strategy not running");
                    
                        // create and initialize strategy
                        let mut strategy = Strategy::new(
                            strategy_name.clone(),
                            strategy_script.clone(),
                            strategy_settings.clone()
                        );
                        strategy.initialize();

                        // add strategy to list of running strategies
                        strategies.add(&strategy_name.clone(), strategy.clone());

                        // return strategy
                        strategy.clone()
                    }
                };

                // start loop through each iteration
                // track orders
                // output orders
                // output performance metrics
                strategy.start();

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