use std::collections::HashMap;
use server::storage::Storage;
use server::QueryStatus;
use server::definition::Definition;
use server::selection::Selection;

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

    pub fn add(&mut self, storage: Storage) -> Result<QueryStatus, QueryStatus>{
        if self.storages.contains_key(storage.get_name().as_str()){
            return Err(QueryStatus::StorageAlreadyExists);
        }
        self.storages.insert(storage.get_name().to_string(), storage);
        return Ok(QueryStatus::StorageCreated);
    }

    pub fn search_in_definition(&mut self, definition: Definition, selection: Selection){
    }

    pub fn get_storage_by_name(&mut self, storage_name: &str) -> Result<&mut Storage, QueryStatus>{
        let possible_definition = self.storages.get_mut(storage_name);
        if possible_definition.is_none() {
            return Err(QueryStatus::SpecifiedDefinitionDoNotExist)
        }
        return Ok(possible_definition.unwrap());
    }
}

