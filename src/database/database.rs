pub struct Database {
    pub name: String,
    pub engine: DatabaseEngine,
    pub is_connected: bool,
}

impl Database {
    pub fn connect(name: String) -> Database {
        Database {
            name: name,
            engine: DatabaseEngine::default(),
            is_connected: false,
        }
    }
}