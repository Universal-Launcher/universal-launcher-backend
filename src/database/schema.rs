table! {
    modpack_versions (id) {
        id -> Integer,
        modpack_id -> Integer,
        version -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

joinable!(modpack_versions -> modpacks (modpack_id));

allow_tables_to_appear_in_same_query!(
    modpack_versions,
    modpacks,
);
