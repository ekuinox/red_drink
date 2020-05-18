table! {
    permissions (path) {
        path -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    roles_permissions (role_id, permission_path) {
        role_id -> Int4,
        permission_path -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(roles_permissions -> permissions (permission_path));
joinable!(roles_permissions -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    permissions,
    roles,
    roles_permissions,
);
