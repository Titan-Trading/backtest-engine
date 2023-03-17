use std::{collections::HashMap, sync::{Mutex, Arc, RwLock}};
use std::io::Error;
use super::strategy::Strategy;


#[derive(Clone)]
pub struct StrategyManager {
    pub strategies: HashMap<String, Strategy>,
}

impl StrategyManager {
    pub fn new() -> StrategyManager {
        StrategyManager {
            strategies: HashMap::new(),
        }
    }

    pub fn add(&mut self, strategy_name: &String, strategy: Strategy) {
        self.strategies.insert(strategy_name.clone(), strategy);
    }

    pub fn remove(&mut self, strategy_name: &String) {
        self.strategies.remove(strategy_name);
    }

    pub fn get(&self, strategy_name: &String) -> Result<&Strategy, Error> {
        // let strategy = strategy.lock().unwrap();

        let strategy = self.strategies.get(strategy_name).unwrap();

        Ok(strategy)
    }
}