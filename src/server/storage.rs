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
        row.insert("id".to_string(), "0".to_string());
        row.insert("name".to_string(), "Alexander".to_string());
        row.insert("role".to_string(), "administrator".to_string());
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

    pub fn search(&self, selection: &Selection) -> Result<Vec<HashMap<String, String>>, QueryStatus>{
        let mut response: Vec<HashMap<String, String>> = vec![];
        let mut data: HashMap<String, String>;
        // get only requested fields
        for row in self.data.iter() {
            data = HashMap::new();
            for key in selection.fields.iter() {
                // If it is required to show all fields
                if key.eq("*") {
                    data = row.clone();
                    break;
                }else{
                    let value = row.get(key.as_str()).unwrap().to_string();
                    data.insert(key.to_string(), value);
                }
            }
            response.push(data);
        }
        return Ok(response);
    }
}

