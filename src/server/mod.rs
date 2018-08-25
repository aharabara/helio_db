use std::collections::HashMap;

pub mod query;
pub mod storage;
pub mod database;
pub mod definition;

pub type FieldMap = HashMap<String, FieldType>;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum DefinitionStatus {
    NoFields,
    NoName,
    InvalidFieldType,
    InvalidInvalidQuery,
    General,
    Good
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum FieldType {
    INTEGER,
    FLOAT,
    STRING,
}