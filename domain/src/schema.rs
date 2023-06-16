// @generated automatically by Diesel CLI.

diesel::table! {
    register_business (id) {
        id -> Int4,
        #[max_length = 100]
        app_name -> Varchar,
        app_logo -> Bytea,
        #[max_length = 100]
        app_url -> Varchar,
        #[max_length = 32]
        api_id -> Varchar,
        #[max_length = 32]
        api_secret -> Varchar,
        notify_customer -> Nullable<Bool>,
        notify_email -> Nullable<Bool>,
        #[max_length = 100]
        set_emails -> Varchar,
        notify_webhook -> Nullable<Bool>,
        #[max_length = 100]
        set_webhook -> Varchar,
        #[max_length = 100]
        link_url_pay -> Nullable<Varchar>,
        link_timeout -> Nullable<Int4>,
        link_amout -> Nullable<Bool>,
        link_count -> Nullable<Bool>,
        ask_name -> Nullable<Bool>,
        ask_mobile -> Nullable<Bool>,
        ask_email -> Nullable<Bool>,
        ask_address -> Nullable<Bool>,
    }
}

