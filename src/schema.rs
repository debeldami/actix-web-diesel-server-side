// @generated automatically by Diesel CLI.

diesel::table! {
    cats (id) {
        id -> Int4,
        name -> Varchar,
        image_path -> Varchar,
    }
}
