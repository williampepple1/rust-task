// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Nullable<Text>,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
