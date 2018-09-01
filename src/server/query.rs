extern crate serde_json;

use serde_json::Value;
use server::definition::Definition;
use server::QueryStatus;
use server::storage::Storage;
use serde_json::Map;
use server::database::Database;
use server::selection::Selection;
use server::insertion::Insertion;

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
        let possible_modification;

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
        if Query::has_query_type("select", query_object) {
            possible_selection = Selection::from_json_object(query_object.get("select").unwrap());
            if possible_selection.is_err() {
                return Err(possible_selection.err().unwrap());
            }
        }else{
            possible_selection = Err(QueryStatus::NoSelection)
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

        let query = Query {
            definition  : possible_definition.ok(),
            selection   : possible_selection.ok(),
            insertion   : possible_insertion.ok(),
        };

        return Ok(query);
    }

    pub fn execute(self, database: &mut Database) -> Result<String, QueryStatus> {
        // Handle definition query
        if self.definition.is_some() {
            let definition = self.definition.unwrap();
            database.add(Storage::new(definition))
        }

        // Handle selection query
        if self.selection.is_some() {
            let selection = self.selection.unwrap();
            // @todo move to selection method
            let possible_storage = database.get_storage_by_name(selection.storage.as_ref());
            if possible_storage.is_err() {
                return Err(possible_storage.err().unwrap());
            }
            let result = possible_storage.unwrap().search(&selection);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let rows = result.ok().unwrap().to_owned();
            let json = serde_json::to_string(&rows).unwrap();

            return Ok(json);
        }

        return Ok("{\"status\" :\"Something went wrong\" }".to_string());
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
