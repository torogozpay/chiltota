// application/src/test/delete.rs

use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Business;

pub fn delete_business(model_id: i32) -> Result<Vec<Business>, NotFound<String>> {
    use domain::schema::businesses::dsl::*;
    use domain::schema::businesses;

    let response: Response;

    let num_deleted = match diesel::delete(businesses.filter(id_business.eq(model_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting business with id {} - {}", model_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match businesses::table.select(businesses::all_columns).load::<Business>(&mut establish_connection()) {
            Ok(mut businesses_) => {
                businesses_.sort();
                Ok(businesses_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no business with id {}", model_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}