use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use serde_json::Map;
use std::string::String;
use server::FieldMap;
use server::FieldType;
use server::DefinitionStatus;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Definition {
    pub name: String,
    pub fields: FieldMap,
}

impl Definition {
    fn new(name: String, fields: FieldMap) -> Definition {
        Definition { name, fields }
    }
    pub fn from_json_object(obj: &Value) -> Result<Definition, DefinitionStatus> {
        let map = obj.as_object().unwrap();
        let result = Definition::validate(map);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let status = result.unwrap();
        // @fixme refactor it
        match status {
            DefinitionStatus::Good => { /* is a valid definition */ }
            _ => { return Err(status); }
        };
        // handle name
        let possible_name = map["name"].as_str();
        if possible_name.is_none() {
            return Err(DefinitionStatus::NoName);
        }
        let name = possible_name.unwrap().to_string();
        let possible_object = map["fields"].as_object();
        if possible_object.is_none() {
            return Err(DefinitionStatus::NoFields);
        }
        let object = possible_object.unwrap();
        let possible_fields = Definition::object_to_fields(object);
        if possible_fields.is_err() {
            return Err(possible_fields.err().unwrap());
        }
        let fields = possible_fields.unwrap();

        Ok(Definition::new(name, fields))
    }

    fn validate(map: &Map<String, Value>) -> Result<DefinitionStatus, DefinitionStatus> {
        if !map.contains_key("name") {
            return Err(DefinitionStatus::NoName);
        }
        if !map.contains_key("fields") {
            return Err(DefinitionStatus::NoFields);
        }
        Ok(DefinitionStatus::Good)
    }

    fn object_to_fields(map: &Map<String, Value>) -> Result<FieldMap, DefinitionStatus> {
        let mut fields: FieldMap = HashMap::new();
        for (name, field_type) in map {
            match field_type.as_str().unwrap().as_ref() {
                "integer" => {
                    fields.insert(name.to_string(), FieldType::INTEGER);
                }
                "float" => {
                    fields.insert(name.to_string(), FieldType::FLOAT);
                }
                "string" => {
                    fields.insert(name.to_string(), FieldType::STRING);
                }
                _ => {
                    return Err(DefinitionStatus::InvalidFieldType);
                }
            }
        }
        Ok(fields)
    }

    pub fn get_name(&self) -> String{
        return self.name.as_str().to_string();
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:?})", self.name, self.fields)
    }
}
