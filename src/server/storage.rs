use server::definition::Definition;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Storage{
    definition : Definition,

    /* @todo try to split in types for better performance? memory usage?*/
    data : Vec<HashMap<String, String>>,
}

impl Storage {
    pub fn new(definition: Definition)-> Storage {
        let data: Vec<HashMap<String, String>> = Vec::new();
        Storage{
            definition,
            data
        }
    }

    pub fn get_name(&self) -> String{
        let name = self.definition.get_name();
        return name;
    }
}

