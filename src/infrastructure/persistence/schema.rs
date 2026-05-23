diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Text,
        user_id -> Text,
        key_hash -> Text,
        label -> Nullable<Text>,
        revoked -> Bool,
    }
}

diesel::joinable!(api_keys -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, api_keys);
