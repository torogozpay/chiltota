// application/src/test/delete.rs

use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Test;

pub fn delete_test(test_id: i32) -> Result<Vec<Test>, NotFound<String>> {
    use domain::schema::tests::dsl::*;
    use domain::schema::tests;

    let response: Response;

    let num_deleted = match diesel::delete(tests.filter(id.eq(test_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting test with id {} - {}", test_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match tests::table.select(tests::all_columns).load::<Test>(&mut establish_connection()) {
            Ok(mut tests_) => {
                tests_.sort();
                Ok(tests_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no test with id {}", test_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}