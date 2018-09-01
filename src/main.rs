extern crate tiny_http;
extern crate serde_json;
extern crate helio_db;


use tiny_http::{Server, Response};
use tiny_http::Request;
use serde_json::Value;
use serde_json::Error;
use helio_db::server::query::Query;
use std::io::Cursor;
use std::borrow::BorrowMut;
use helio_db::server::database::Database;
use helio_db::server::QueryStatus;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    let mut database = Database::new();
    for mut request in server.incoming_requests() {
        println!("received request! method:\n{:#?}\nurl: {:#?}\nheaders: {:?}\n",
                 request.method(),
                 request.url(),
                 request.headers()
        );
        let json = request.get_json();
        let response = request.get_response(json, database.borrow_mut());
        /*@todo add bad response (>400)*/
        if response.is_err() {
            println!("Error : {:#?}", response.err().unwrap());
        }else {
            request.respond(response.unwrap()).unwrap();
        }
        println!("Storage: {:#?}", database);
    }
}


pub trait RequestExt {
    fn get_json(&mut self) -> Result<Value, Error>;
    fn get_response(&mut self, json: Result<Value, Error>, database : &mut Database) -> Result<Response<Cursor<Vec<u8>>>, QueryStatus>;
}

impl RequestExt for Request {
    fn get_json(&mut self) -> Result<Value, Error> {
        let reader = self.as_reader();
        let result = serde_json::from_reader(reader);
        return result;
    }

    // handle request
    /*@todo rewrite to json response */
    fn get_response(&mut self,  json: Result<Value, Error>, database : &mut Database) -> Result<Response<Cursor<Vec<u8>>>, QueryStatus> {
        let response;
        match json {
            Ok(value) => {
                let query = Query::from_json_object(value);
                if query.is_err() {
                    response = Response::from_string(format!("ERROR: {:?}\n", query.err().unwrap()));
                } else {
                    let possible_json_response = query.unwrap().execute(database);
                    if possible_json_response.is_err() {
                        return Err(possible_json_response.err().unwrap());
                    }
                    response = Response::from_string(possible_json_response.unwrap());
                }
            }
            Err(error) => {
                response = Response::from_string(format!("Invalid query: \n{:?}", error));
            }
        }
        return Ok(response);
    }
}