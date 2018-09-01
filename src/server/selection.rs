use serde_json::Value;
use serde_json::Map;
use std::string::String;
use server::QueryStatus;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Selection {
    fields: Vec<String>,
    pub definition_name: String,
//    where_clause: WhereClause,
}

impl Selection {
    fn new(
        definition_name: String,
        fields: Vec<String>
//           , where_clause: WhereClause
    ) -> Selection {
        Selection {
            fields,
            definition_name,
//            where_clause
        }
    }
    pub fn from_json_object(obj: &Value) -> Result<Selection, QueryStatus> {
        let map = obj.as_object().unwrap();
        let result = Selection::validate(map);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
// handle name
        let possible_name = map["definition_name"].as_str();
        if possible_name.is_none() {
            return Err(QueryStatus::NoName);
        }

        let possible_fields = map["fields"].as_array();

        return Ok(Selection::new(possible_name.unwrap().to_string(), vec!["*".to_string()]));
        // @todo array<value> => array<str>
//        return Selection::new(possible_name.unwrap().to_string(), possible_fields.unwrap())
    }

    fn validate(map: &Map<String, Value>) -> Result<QueryStatus, QueryStatus> {
        if !map.contains_key("definition_name") {
            return Err(QueryStatus::NoDefinitionSpecified);
        }
        if map.contains_key("fields") {
            let fields = map["fields"].as_array();
            if fields.is_none() {
                return Err(QueryStatus::InvalidSelectionFieldsFormat);
            }
        }
        Ok(QueryStatus::Good)
    }
}