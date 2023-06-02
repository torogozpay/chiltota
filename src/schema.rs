// @generated automatically by Diesel CLI.

pub mod ecommerce {
    diesel::table! {
        ecommerce.tests (id) {
            id -> Int4,
            codigo -> Varchar,
            nombre -> Varchar,
        }
    }
}
