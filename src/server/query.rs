extern crate serde_json;

use serde_json::Value;
use server::QueryStatus;
use server::storage::Storage;
use serde_json::Map;
use std::collections::HashMap;
use server::operation::definition::Definition;
use server::operation::selection::Selection;
use server::operation::insertion::Insertion;
use server::database::Database;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Query {
    definition  : Option<Definition>,
    selection   : Option<Selection>,
    insertion   : Option<Insertion>,
}

impl Query {
    pub fn from_json_object(query_request: Value) -> Result<Query, QueryStatus> {
        let possible_definition;
        let possible_selection;
        let possible_insertion;

        let possible_query_obj = query_request.as_object();
        if possible_query_obj.is_none() {
            return Err(QueryStatus::InvalidQuery)
        }

        let query_object = possible_query_obj.unwrap();

        // if there is any storage definition clause
        if Query::has_query_type("define",query_object) {
            possible_definition = Definition::from_json_object(query_object.get("define").unwrap());
            if possible_definition.is_err() {
                return Err(possible_definition.err().unwrap());
            }
        }else{
            possible_definition = Err(QueryStatus::NoDefinition)
        }

        // if there is any data selection clause
        if Query::has_query_type("insert", query_object) {
            possible_insertion = Insertion::from_json_object(query_object.get("insert").unwrap());
            if possible_insertion.is_err() {
                return Err(possible_insertion.err().unwrap());
            }
        }else{
            possible_insertion = Err(QueryStatus::NoModification)
        }

        // if there is any data selection clause
        if Query::has_query_type("select", query_object) {
            possible_selection = Selection::from_json_object(query_object.get("select").unwrap());
            if possible_selection.is_err() {
                return Err(possible_selection.err().unwrap());
            }
        }else{
            possible_selection = Err(QueryStatus::NoSelection)
        }

        let query = Query {
            definition  : possible_definition.ok(),
            insertion   : possible_insertion.ok(),
            selection   : possible_selection.ok(),
        };

        return Ok(query);
    }

    pub fn execute(self, database: &mut Database) -> HashMap<String, Value> {
        let mut response:HashMap<String, Value> = HashMap::new();
        // Handle definition query
        if self.definition.is_some() {
            let definition = self.definition.unwrap();
            let result = database.add(Storage::new(definition));
            let status = if result.is_err() { result.err() } else { result.ok() };
            let value = serde_json::to_value(status.unwrap().to_string()).unwrap();
            response.insert("define".to_string(), value);
        }

        // Handle definition query
        if self.insertion.is_some() {
            let insertion = self.insertion.unwrap();
            // @todo move to insertion method
            let possible_storage = database.get_storage_by_name(insertion.storage.as_ref());
            if possible_storage.is_err() {
                let result = serde_json::to_value(possible_storage.err().unwrap().to_string());
                response.insert("insert".to_string(), result.unwrap());
            }else{
                let mut storage = possible_storage.unwrap();
                let result = insertion.execute(&mut storage);
                if result.is_err(){
                    let value = serde_json::to_value(result.err().unwrap().to_string()).unwrap();
                    response.insert("insert".to_string(), value);
                }else{
                    let value = serde_json::to_value(result.ok().unwrap().to_string()).unwrap();
                    response.insert("insert".to_string(), value);
                }
            }
        }

        // Handle selection query
        if self.selection.is_some() {
            let selection = self.selection.unwrap();
            // @todo move to selection method
            let possible_storage = database.get_storage_by_name(selection.storage.as_ref());
            if possible_storage.is_err() {
                let result = serde_json::to_value(possible_storage.err().unwrap().to_string());
                response.insert("select".to_string(), result.unwrap());
            }else{
                /* @TODO stopped here*/
                let result = possible_storage.unwrap().search(&selection);
                if result.is_err() {
                    let result = serde_json::to_value(result.err().unwrap().to_string());
                    response.insert("select".to_string(), result.unwrap());
                }else{
                    let rows = result.ok().unwrap().to_owned();
                    let rows = serde_json::to_value(&rows).unwrap();
                    response.insert("select".to_string(), rows);
                }
            }
        }
        return response;
    }
    pub fn has_query_type(query_type : &str, query_obj: &Map<String, Value>) -> bool {
        let possible_define = query_obj.get(query_type);
        if possible_define.is_none() {
            println!("Query doesn't contain any '{}' clause.", query_type);
            return false;
        }
        if possible_define.unwrap().as_object().is_none(){
            println!("Query '{}' clause is empty.", query_type);
            return false;
        }
        return true;
    }
}
