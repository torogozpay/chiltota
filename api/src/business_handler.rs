// api/src/test_handler.rs

use shared::generate_numbers::{gen_api_id, gen_api_secret};
use shared::response_models::{Response, ResponseBody};
use application::business::{create, update, read, delete}; 
use domain::models::{Business, NewBusiness};
use rocket::{get, post, put};
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
    let id = gen_api_id();
    let secret = gen_api_secret();
    let mut mybusiness = business;
    mybusiness.api_id = &id;
    mybusiness.api_secret = &secret;

    create::create_business(mybusiness)
}

#[put("/updBusiness", format = "application/json", data = "<business>")]
pub fn update_business_handler(business: Json<NewBusiness>) -> Result<String, NotFound<String>> {
    let businesses = update::update_business(business)?;
    let response = Response { body: ResponseBody::Business(businesses) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/deleteBusiness/<model_id>")]
pub fn delete_business_handler(model_id: i32) -> Result<String, NotFound<String>> {
    let businesses = delete::delete_business(model_id)?;
    let response = Response { body: ResponseBody::Businesses(businesses) };

    Ok(serde_json::to_string(&response).unwrap())
}