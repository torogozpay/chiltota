// domain/src/models.rs

 use crate::schema::tests;
 use diesel::prelude::*;
 use rocket::serde::{Deserialize, Serialize};
 use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
 
 #[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
 pub struct Test {
    pub id: i32,
    pub codigo: String,
    pub nombre: String,
 }
 
 #[derive(Insertable, Deserialize)]
 #[serde(crate = "rocket::serde")]
 #[diesel(table_name = tests)]
 pub struct NewTest<'a> {
     pub codigo: &'a str,
     pub nombre: &'a str,
 }