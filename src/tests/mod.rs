#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::ecommerce::tests;

pub mod handler;
pub mod router;
pub mod repository;


#[derive(Queryable, AsChangeset, Serialize, Deserialize, QueryableByName, Debug)]
#[table_name = "tests"]
pub struct Test {
    pub id: i32,
    pub codigo: String,
    pub nombre: String,
}