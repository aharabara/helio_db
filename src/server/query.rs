use serde_json::Value;
use server::definition::Definition;
use std::fmt;
use server::DefinitionStatus;
use server::storage::Storage;
use serde_json::Map;
use server::database::Database;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum QueryType {
    CREATE,
    READ,
    UPDATE,
    DELETE,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum QueryStatus {
    NEW,
    EXECUTED,
    FAILED,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Query {
    query_status: QueryStatus,
    query_type: QueryType,
    definition: Option<Definition>,
}

impl Query {
    pub fn from_json_object(query_request: Value) -> Result<Query, DefinitionStatus> {
        let result;
        let possible_query_obj = query_request.as_object();
        if possible_query_obj.is_none() {
            return Err(DefinitionStatus::InvalidInvalidQuery)
        }

        let query_object = possible_query_obj.unwrap();
        if Query::has_definition(query_object) {
            result = Definition::from_json_object(query_object.get("define").unwrap());
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }else{
            unimplemented!("\nIs not a creation query!\n")
        }

        let query = Query {
            query_status: QueryStatus::NEW,
            query_type: QueryType::CREATE,
            definition: result.ok(),
        };

        return Ok(query);
    }

    pub fn execute(self, database: &mut Database) {
        match self.query_type {
            QueryType::CREATE => {
                let definition = self.definition.unwrap();
                database.add(Storage::new(definition))
            },
            _ => {
                unimplemented!("Implement more then just definition creation.")
            }
        }
    }
    pub fn has_definition(query_obj: &Map<String, Value>) -> bool {
        let possible_define = query_obj.get("define");
        if possible_define.is_none() {
            println!("No define clause.");
            return false;
        }
        if possible_define.unwrap().as_object().is_none(){
            println!("Definition clause is empty.");
            return false;
        }
        return true;
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.definition.is_none() {
            return write!(f, "({}, {})", self.query_type, self.query_status)
        }
        let definition = self.definition.as_ref().unwrap();
        return write!(f, "({}, {}, {})", self.query_type, self.query_status, definition)
    }
}

impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryType::CREATE => write!(f, "create"),
            QueryType::READ => write!(f, "read"),
            QueryType::UPDATE => write!(f, "update"),
            QueryType::DELETE => write!(f, "delete"),
        }
    }
}

impl fmt::Display for QueryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryStatus::EXECUTED => write!(f, "execute"),
            QueryStatus::FAILED => write!(f, "faile"),
            QueryStatus::NEW => write!(f, "new"),
        }
    }
}
