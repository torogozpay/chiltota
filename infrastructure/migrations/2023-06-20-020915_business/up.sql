-- Your SQL goes here
CREATE TABLE businesses (
 id_business serial primary key,
 app_name varchar(100) not null,
 app_logo varchar(100) not null,
 app_url varchar(100) not null,
 api_id varchar(32) not null,
 api_secret varchar(32) not null,
 id_workspace uuid not null,
 notify_customer boolean not null default false,
 notify_email boolean not null default false, 
 set_emails varchar(100),
 notify_webhook boolean not null default false, 
 set_webhook varchar(100),
 link_url_pay varchar(100),
 link_timeout int not null default 0,
 link_amount boolean not null default false,
 link_count boolean not null default false,
 ask_name boolean not null default false, 
 ask_mobile boolean not null default false, 
 ask_email boolean not null default false, 
 ask_address boolean not null  default false
);


CREATE TABLE invoices (
 id_invoice serial primary key,
 id_business integer not null,
 payment_address varchar(100) not null,
 description varchar(250) not null,
 amount numeric(18,2) not null default 0,
 invoice_date timestamp with time zone not null,
 first_name varchar(50) not null,
 last_name varchar(50) not null,
 email varchar(60) not null,
 phone_number varchar(25) not null,
 address varchar(100) not null,
 city varchar(50) not null,
 id_country varchar(20) not null,
 id_region varchar(20) not null,
 postal_code varchar(20) not null,
 url_redirect varchar(100) not null
);


CREATE TABLE invoices_det (
 id_invoice_det serial primary key,
 id_invoice integer not null,
 product_code varchar(30) not null,
 quantity numeric(18,2) not null default 0,
 amount numeric(18,2) not null default 0
);

/*
alter table invoices
   add constraint fk_invoices_businesses foreign key (id_business)
      references businesses (id_business)
      on delete restrict on update restrict;
	  
alter table invoices_det
   add constraint fk_invoices_det_invoices foreign key (id_invoice)
      references invoices (id_invoice)
      on delete restrict on update restrict;	  



create unique index idx_cn_businesses on businesses (
 api_id
);

create unique index idx_cn_invoices on invoices (
 id_business,
 payment_address,
 invoice_date,	
 amount	
);

create unique index idx_cn_invoices_det on invoices_det (
 id_invoice,
 product_code	
);
*/