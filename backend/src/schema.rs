table! {
    github_users (github_id) {
        github_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

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

table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(github_users -> users (user_id));
joinable!(roles_permissions -> permissions (permission_path));
joinable!(roles_permissions -> roles (role_id));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    github_users,
    permissions,
    roles,
    roles_permissions,
    users,
    users_roles,
);
