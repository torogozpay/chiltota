// application/src/test/update.rs

use domain::models::{Business, UpdBusiness};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::response::status::NotFound;

pub fn update_business(business: Json<UpdBusiness>) -> Result<Business, NotFound<String>> {
    use domain::schema::businesses::dsl::*;

    let id = business.id_business;
    let business = business.into_inner();

    match diesel::update(businesses.filter(id_business.eq(id))).set(business).get_result::<Business>(&mut establish_connection()) {
        Ok(business) => Ok(business), 
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error update business with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }

}