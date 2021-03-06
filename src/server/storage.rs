use std::collections::HashMap;
use server::QueryStatus;
use server::FieldType;
use server::operation::definition::Definition;
use server::operation::selection::Selection;

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

    pub fn insert(&mut self, row: HashMap<String, String>) -> bool{
        if self.is_valid_row(&row) {
            self.data.push(row.clone());
            return true;
        }
        return false;
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
    pub fn is_valid_row(&self, row: &HashMap<String, String>) -> bool{
        for (field, field_type) in self.definition.fields.iter() {
            if !row.contains_key(field){
                return false
            }
            let value = row.get(field).unwrap();
            match field_type {
                FieldType::STRING => { /* skip, because it is already string */}
                FieldType::INTEGER => {
                    let result = value.parse::<i32>();
                    if result.is_err() {
                        return false;
                    }
                }
                FieldType::FLOAT => {
                    let result = value.parse::<f32>();
                    if result.is_err() {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

