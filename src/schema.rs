// @generated automatically by Diesel CLI.
// TODO: Currently using Varbinaries for salt and hashes, which shouldn't be required
diesel::table! {
    credentials (id) {
        id -> Integer,
        #[max_length = 255]
        password_hash -> Varbinary,
        #[max_length = 255]
        salt -> Varbinary,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    users,
);
