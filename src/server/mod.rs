use std::collections::HashMap;

pub mod query;
pub mod storage;
pub mod database;
pub mod definition;
pub mod selection;
pub mod where_clause;

pub type FieldMap = HashMap<String, FieldType>;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum QueryStatus {
    NoDefinition,
    NoFields,
    NoName,
    InvalidFieldType,
    InvalidQuery,
    General,
    Good,
    NoDefinitionSpecified,
    InvalidSelectionFieldsFormat,
    SpecifiedDefinitionDoNotExist
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum FieldType {
    INTEGER,
    FLOAT,
    STRING,
}