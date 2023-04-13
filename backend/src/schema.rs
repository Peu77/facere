// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    users,
);
