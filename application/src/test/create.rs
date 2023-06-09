// application/src/test/create.rs

use domain::models::{Test, NewTest};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_test(test: Json<NewTest>) -> Created<String> {
    use domain::schema::tests;

    let test = test.into_inner();

    match diesel::insert_into(tests::table).values(&test).get_result::<Test>(&mut establish_connection()) {
        Ok(test) => {
            let response = Response { body: ResponseBody::Test(test) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}