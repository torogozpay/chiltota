-- Your SQL goes here
CREATE TABLE register_business (
 id serial primary key,
 app_name varchar(100) not null,
 app_logo bytea not null,
 app_url varchar(100) not null,
 api_id varchar(32) not null,
 api_secret varchar(32) not null,
 notify_customer boolean,
 notify_email boolean, 
 set_emails varchar(100) not null,
 notify_webhook boolean, 
 set_webhook varchar(100) not null,
 link_url_pay varchar(100) null,
 link_timeout int null,
 link_amout boolean,
 link_count boolean,
 ask_name boolean, 
 ask_mobile boolean, 
 ask_email boolean, 
 ask_address boolean 
);
