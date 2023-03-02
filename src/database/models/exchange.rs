

#[derive(Debug, Clone)]
pub struct Exchange {
    pub name: String,
}

impl Exchange {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    pub fn new_with(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}