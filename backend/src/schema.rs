table! {
    accessibles (role_id, permission_path, resource_id) {
        role_id -> Int4,
        permission_path -> Varchar,
        resource_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    actions (id) {
        id -> Int4,
        kind -> Varchar,
        descriptor -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    assignments (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    github_accounts (github_id) {
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
    resources (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
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
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(accessibles -> permissions (permission_path));
joinable!(accessibles -> resources (resource_id));
joinable!(accessibles -> roles (role_id));
joinable!(assignments -> roles (role_id));
joinable!(assignments -> users (user_id));
joinable!(github_accounts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accessibles,
    actions,
    assignments,
    github_accounts,
    permissions,
    resources,
    roles,
    users,
);
