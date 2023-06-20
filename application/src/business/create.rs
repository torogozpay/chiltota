// application/src/test/create.rs

use domain::models::{Business, NewBusiness};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_business(business: Json<NewBusiness>) -> Created<String> {
    use domain::schema::businesses;

    let business = business.into_inner();

    match diesel::insert_into(businesses::table).values(&business).get_result::<Business>(&mut establish_connection()) {
        Ok(business) => {
            let response = Response { body: ResponseBody::Business(business) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }

}