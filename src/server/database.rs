use std::collections::HashMap;
use server::storage::Storage;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Database {
    storages: HashMap<String, Storage>
}

impl Database {
    pub fn new() -> Database{
        let storages: HashMap<String, Storage> = HashMap::new();
        Database{
            storages
        }
    }

    pub fn add(&mut self, storage: Storage){
        self.storages.insert(storage.get_name().to_string(), storage);
    }
}

