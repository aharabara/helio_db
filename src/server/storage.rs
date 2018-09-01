use server::definition::Definition;
use std::collections::HashMap;
use server::selection::Selection;
use server::QueryStatus;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Storage{
    definition : Definition,

    /* @todo try to split in types for better performance? memory usage?*/
    data : Vec<HashMap<String, String>>,
}

impl Storage {
    pub fn new(definition: Definition)-> Storage {
        let mut data: Vec<HashMap<String, String>> = Vec::new();
        let mut row: HashMap<String, String> = HashMap::new();
        /*@todo remove when insert will be done*/
        row.insert("field".to_string(), "value".to_string());
        data.push(row);

        Storage{
            definition,
            data
        }
    }

    pub fn get_name(&self) -> String{
        let name = self.definition.get_name();
        return name;
    }

    pub fn search(&self, selection: &Selection) -> Result<&Vec<HashMap<String, String>>, QueryStatus>{
        return Ok(self.data.as_ref());
    }
}

