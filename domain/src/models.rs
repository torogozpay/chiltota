// domain/src/models.rs

 use crate::schema::*;
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

 #[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
 pub struct Business {
     pub id_business: i32,
     pub app_name: String,
     pub app_logo: String,
     pub app_url: String,
     pub api_id: String,
     pub api_secret: String,
     pub notify_customer: bool,
     pub notify_email: bool,
     pub set_emails: Option<String>,
     pub notify_webhook: bool,
     pub set_webhook: Option<String>,
     pub link_url_pay: Option<String>,
     pub link_timeout: i32,
     pub link_amount: bool,
     pub link_count: bool,
     pub ask_name: bool,
     pub ask_mobile: bool,
     pub ask_email: bool,
     pub ask_address: bool,
 }
 
 #[derive(Insertable, Deserialize)]
 #[serde(crate = "rocket::serde")]
 #[diesel(table_name = businesses)]
 pub struct NewBusiness {
     pub app_name: String,
     pub app_logo: String,
     pub app_url: String,
     pub api_id: String,
     pub api_secret: String,
     pub notify_customer: bool,
     pub notify_email: bool,
     pub set_emails: Option<String>,
     pub notify_webhook: bool,
     pub set_webhook: Option<String>,
     pub link_url_pay: Option<String>,
     pub link_timeout: i32,
     pub link_amount: bool,
     pub link_count: bool,
     pub ask_name: bool,
     pub ask_mobile: bool,
     pub ask_email: bool,
     pub ask_address: bool,
    }