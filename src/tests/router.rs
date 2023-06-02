use rocket;

use crate::connection;
use crate::tests;

use dotenv::dotenv;
use std::env;


pub fn create_routes() { 
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = connection::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/",
            routes![tests::handler::index],
        )       
        .mount(
            "/api/v1/",
            routes![tests::handler::get_all, tests::handler::new_test, tests::handler::find_test],
        ) 
        .launch();   

}