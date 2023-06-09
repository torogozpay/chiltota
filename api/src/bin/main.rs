// api/src/bin/main.rs

#[macro_use] extern crate rocket;
use api::test_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![
            test_handler::list_tests_handler, 
            test_handler::list_test_handler,
            test_handler::create_test_handler,
            test_handler::delete_test_handler, 
        ])
}