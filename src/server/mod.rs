use std::collections::HashMap;

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
    General,
    Good,
    NoDefinitionSpecified,
    InvalidSelectionFieldsFormat,
    SpecifiedDefinitionDoNotExist,
    FieldsClauseShouldContainOnlyStrings,
    DataForInsertionNotSpecified,
    InvalidRowFormat
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum FieldType {
    INTEGER,
    FLOAT,
    STRING,
}