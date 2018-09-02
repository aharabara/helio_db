extern crate serde_json;

use std::collections::HashMap;
use std::fmt;
use tiny_http::Response;
use std::io::Cursor;

pub mod query;
pub mod storage;
pub mod database;
pub mod definition;
pub mod selection;
pub mod insertion;
pub mod where_clause;

pub type FieldMap = HashMap<String, FieldType>;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum QueryStatus {
    NoDefinition,
    NoModification,
    NoSelection,
    NoFields,
    NoName,
    InvalidFieldType,
    InvalidQuery,
    Valid,
    Inserted,
    NoDefinitionSpecified,
    InvalidSelectionFieldsFormat,
    SpecifiedDefinitionDoNotExist,
    FieldsClauseShouldContainOnlyStrings,
    DataForInsertionNotSpecified,
    InvalidRowFormat,
    StorageAlreadyExists,
    StorageCreated,
}

impl QueryStatus {
    pub fn to_response(&self) -> Response<Cursor<Vec<u8>>> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("error".to_string(), self.to_string());
        let result = serde_json::to_string(&map);
        if result.is_err(){
            panic!(result.err());
        }
        Response::from_string(result.ok().unwrap())
    }
}

impl fmt::Display for QueryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum FieldType {
    INTEGER,
    FLOAT,
    STRING,
}