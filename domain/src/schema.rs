// @generated automatically by Diesel CLI.

diesel::table! {
    businesses (id_business) {
        id_business -> Int4,
        #[max_length = 100]
        app_name -> Varchar,
        #[max_length = 100]
        app_logo -> Varchar,
        #[max_length = 100]
        app_url -> Varchar,
        #[max_length = 32]
        api_id -> Varchar,
        #[max_length = 32]
        api_secret -> Varchar,
        id_workspace -> Uuid,
        notify_customer -> Bool,
        notify_email -> Bool,
        #[max_length = 100]
        set_emails -> Nullable<Varchar>,
        notify_webhook -> Bool,
        #[max_length = 100]
        set_webhook -> Nullable<Varchar>,
        #[max_length = 100]
        link_url_pay -> Nullable<Varchar>,
        link_timeout -> Int4,
        link_amount -> Bool,
        link_count -> Bool,
        ask_name -> Bool,
        ask_mobile -> Bool,
        ask_email -> Bool,
        ask_address -> Bool,
    }
}

diesel::table! {
    invoices (id_invoice) {
        id_invoice -> Int4,
        id_business -> Int4,
        #[max_length = 100]
        payment_address -> Varchar,
        #[max_length = 250]
        description -> Varchar,
        amount -> Numeric,
        invoice_date -> Timestamptz,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 60]
        email -> Varchar,
        #[max_length = 25]
        phone_number -> Varchar,
        #[max_length = 100]
        address -> Varchar,
        #[max_length = 50]
        city -> Varchar,
        #[max_length = 20]
        id_country -> Varchar,
        #[max_length = 20]
        id_region -> Varchar,
        #[max_length = 20]
        postal_code -> Varchar,
        #[max_length = 100]
        url_redirect -> Varchar,
    }
}

diesel::table! {
    invoices_det (id_invoice_det) {
        id_invoice_det -> Int4,
        id_invoice -> Int4,
        #[max_length = 30]
        product_code -> Varchar,
        quantity -> Numeric,
        amount -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    businesses,
    invoices,
    invoices_det,
);
