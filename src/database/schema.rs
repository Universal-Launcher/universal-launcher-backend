table! {
    modpacks (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        enabled -> Bool,
    }
}
