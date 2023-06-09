// api/src/test_handler.rs

use shared::response_models::{Response, ResponseBody};
use application::test::{create, read, delete}; 
use domain::models::{Test, NewTest};
use rocket::{get, post};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;



#[get("/getTest")]
pub fn list_tests_handler() -> String {
    // 👇 New function body!
    let tests: Vec<Test> = read::list_tests();
    let response = Response { body: ResponseBody::Tests(tests) };

    serde_json::to_string(&response).unwrap()
}

#[get("/getTest/<test_id>")]
pub fn list_test_handler(test_id: i32) -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let test = read::list_test(test_id)?;
    let response = Response { body: ResponseBody::Test(test) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/newTest", format = "application/json", data = "<test>")]
pub fn create_test_handler(test: Json<NewTest>) -> Created<String> {
    create::create_test(test)
}

#[get("/delete/<test_id>")]
pub fn delete_test_handler(test_id: i32) -> Result<String, NotFound<String>> {
    let tests = delete::delete_test(test_id)?;
    let response = Response { body: ResponseBody::Tests(tests) };

    Ok(serde_json::to_string(&response).unwrap())
}