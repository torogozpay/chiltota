// application/src/test/read.rs

use domain::models::Test;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_test(test_id: i32) -> Result<Test, NotFound<String>> {
    use domain::schema::tests;

    match tests::table.find(test_id).first::<Test>(&mut establish_connection()) {
        Ok(test) => Ok(test),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting test with id {} - {}", test_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_tests() -> Vec<Test> {
    use domain::schema::tests;

    match tests::table.select(tests::all_columns).load::<Test>(&mut establish_connection()) {
        Ok(mut tests) => {
            tests.sort();
            tests
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}