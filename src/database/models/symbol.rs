
// represents a ticker symbol
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub target_currency: String,
    pub base_currency: String,
}

impl Symbol {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            target_currency: "".to_string(),
            base_currency: "".to_string(),
        }
    }

    pub fn new_with(target_currency: String, base_currency: String) -> Self {
        Self {
            name: format!("{}{}", target_currency, base_currency),
            target_currency,
            base_currency,
        }
    }

    pub fn set_target_currency(&mut self, target_currency: String) {
        self.target_currency = target_currency;
    }

    pub fn set_base_currency(&mut self, base_currency: String) {
        self.base_currency = base_currency;
    }
}