// api/src/test_handler.rs

use shared::response_models::{Response, ResponseBody};
use application::business::{create, read, delete}; 
use domain::models::{Business, NewBusiness};
use rocket::{get, post};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;



#[get("/getBusiness")]
pub fn list_businesses_handler() -> String {
    // 👇 New function body!
    let businesses: Vec<Business> = read::list_businesses();
    let response = Response { body: ResponseBody::Businesses(businesses) };

    serde_json::to_string(&response).unwrap()
}

#[get("/getBusiness/<model_id>")]
pub fn list_business_handler(model_id: i32) -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let business = read::list_business(model_id)?;
    let response = Response { body: ResponseBody::Business(business) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/newBusiness", format = "application/json", data = "<business>")]
pub fn create_business_handler(business: Json<NewBusiness>) -> Created<String> {
    create::create_business(business)
}

#[get("/deleteBusiness/<model_id>")]
pub fn delete_business_handler(model_id: i32) -> Result<String, NotFound<String>> {
    let businesses = delete::delete_business(model_id)?;
    let response = Response { body: ResponseBody::Businesses(businesses) };

    Ok(serde_json::to_string(&response).unwrap())
}