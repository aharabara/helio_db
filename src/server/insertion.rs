use serde_json::Value;
use serde_json::Map;
use std::string::String;
use server::QueryStatus;
use std::collections::HashMap;
use server::storage::Storage;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Insertion {
    pub storage: String,
    data: Vec<HashMap<String, String>>
}

impl Insertion {
    fn new(
        storage: String,
        data: Vec<HashMap<String, String>>
    ) -> Insertion {
        Insertion { storage, data,}
    }
    pub fn from_json_object(obj: &Value) -> Result<Insertion, QueryStatus> {
        let map = obj.as_object().unwrap();
        let result = Insertion::validate(map);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        // handle name
        let possible_name = map["storage"].as_str().unwrap();

        let possible_rows = map["data"].as_array().unwrap();
        let rows = Insertion::value_to_data(possible_rows);
        if rows.is_err() {
            return Err(rows.err().unwrap());
        }

        return Ok(Insertion::new(possible_name.to_string(), rows.unwrap()));
    }

    fn value_to_data(data: &Vec<Value>) -> Result<Vec<HashMap<String, String>>, QueryStatus> {
        let mut rows:Vec<HashMap<String, String>> = vec![];
        let mut row: HashMap<String, String>;
        for item in data.iter() {
            let map = item.as_object();
            if map.is_none() {
                /*@todo return with line and field if possible*/
                return Err(QueryStatus::InvalidRowFormat);
            }
            row = HashMap::new();
            for (key, field) in map.unwrap().iter() {
                let possible_value = field.as_str();
                if possible_value.is_none() {
                    return Err(QueryStatus::InvalidRowFormat);
                }
                row.insert(key.to_string(), possible_value.unwrap().to_string());
            }
            rows.push(row);
        }
        Ok(rows)
    }

    fn validate(map: &Map<String, Value>) -> Result<QueryStatus, QueryStatus> {
        if !map.contains_key("storage") {
            return Err(QueryStatus::NoDefinitionSpecified);
        }
        if map.contains_key("data") {
            let data = map["data"].as_array();
            if data.is_none() {
                return Err(QueryStatus::DataForInsertionNotSpecified);
            }
        }
        Ok(QueryStatus::Valid)
    }

    pub fn execute(&self, storage: &mut Storage) -> Result<QueryStatus, QueryStatus>{
        for row in self.data.iter() {
            if !storage.insert(row.to_owned()) {
                return Err(QueryStatus::InvalidRowFormat);
            }
        }
        return Ok(QueryStatus::Inserted);
    }
}