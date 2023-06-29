// api/src/bin/main.rs

#[macro_use] extern crate rocket;
use api::business_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![
            business_handler::list_businesses_handler, 
            business_handler::list_business_handler,
            business_handler::create_business_handler,
            business_handler::update_business_handler,
            business_handler::delete_business_handler,              
        ])
}