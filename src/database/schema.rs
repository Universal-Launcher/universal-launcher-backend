table! {
    modpack_versions (id) {
        id -> Int4,
        modpack_id -> Int4,
        version -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    modpacks (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        enabled -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        email_verified -> Bool,
    }
}

joinable!(modpack_versions -> modpacks (modpack_id));

allow_tables_to_appear_in_same_query!(modpack_versions, modpacks, users,);
