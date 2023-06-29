// domain/src/models.rs
 extern crate uuid;

 use crate::schema::*;
 use diesel::prelude::*;
 use rocket::serde::{Deserialize, Serialize};
 use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
 use uuid::Uuid;


 
  #[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
 pub struct Business {
     pub id_business: i32,
     pub app_name: String,
     pub app_logo: String,
     pub app_url: String,
     pub api_id: String,
     pub api_secret: String,
     #[serde(with = "my_uuid")]
     pub id_workspace: Uuid,
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
 pub struct NewBusiness <'a> {
     pub app_name: &'a str,
     pub app_logo: &'a str,
     pub app_url: &'a str,
     pub api_id: &'a str,
     pub api_secret: &'a str,
     #[serde(with = "my_uuid")]
     pub id_workspace: Uuid,
     pub notify_customer: bool,
     pub notify_email: bool,
     pub set_emails: Option<&'a str>,
     pub notify_webhook: bool,
     pub set_webhook: Option<&'a str>,
     pub link_url_pay: Option<&'a str>,
     pub link_timeout: i32,
     pub link_amount: bool,
     pub link_count: bool,
     pub ask_name: bool,
     pub ask_mobile: bool,
     pub ask_email: bool,
     pub ask_address: bool,
    }

    #[derive(AsChangeset, Deserialize)]
    #[serde(crate = "rocket::serde")]
    #[diesel(table_name = businesses)]
    pub struct UpdBusiness <'a> {
        pub id_business: i32,
        pub app_name: &'a str,
        pub app_logo: &'a str,
        pub app_url: &'a str,
        pub api_id: &'a str,
        pub api_secret: &'a str,
        #[serde(with = "my_uuid")]
        pub id_workspace: Uuid,
        pub notify_customer: bool,
        pub notify_email: bool,
        pub set_emails: Option<&'a str>,
        pub notify_webhook: bool,
        pub set_webhook: Option<&'a str>,
        pub link_url_pay: Option<&'a str>,
        pub link_timeout: i32,
        pub link_amount: bool,
        pub link_count: bool,
        pub ask_name: bool,
        pub ask_mobile: bool,
        pub ask_email: bool,
        pub ask_address: bool,
       }
    mod my_uuid {
        use uuid::Uuid;
        use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
        use std::str::FromStr;
    
        pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            val.to_string().serialize(serializer)
        }
    
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
        where
            D: Deserializer<'de>,
        {
            let val: &str = Deserialize::deserialize(deserializer)?;
            Uuid::from_str(val).map_err(D::Error::custom)
        }
    }