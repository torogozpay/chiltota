// shared/src/response_models.rs

use domain::models::*;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Test(Test),
    Tests(Vec<Test>),
    Business(Business),
    Businesses(Vec<Business>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}