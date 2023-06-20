// application/src/test/read.rs

use domain::models::Business;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_business(model_id: i32) -> Result<Business, NotFound<String>> {
    use domain::schema::businesses;

    match businesses::table.find(model_id).first::<Business>(&mut establish_connection()) {
        Ok(business) => Ok(business),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting business with id {} - {}", model_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_businesses() -> Vec<Business> {
    use domain::schema::businesses;

    match businesses::table.select(businesses::all_columns).load::<Business>(&mut establish_connection()) {
        Ok(mut businesses) => {
            businesses.sort();
            businesses
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}