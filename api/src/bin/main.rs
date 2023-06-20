// api/src/bin/main.rs

#[macro_use] extern crate rocket;
use api::test_handler;
use api::business_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![
            test_handler::list_tests_handler, 
            test_handler::list_test_handler,
            test_handler::create_test_handler,
            test_handler::delete_test_handler,
            business_handler::list_businesses_handler, 
            business_handler::list_business_handler,
            business_handler::create_business_handler,
            business_handler::delete_business_handler,              
        ])
}