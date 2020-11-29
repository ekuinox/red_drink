table! {
    actions (id) {
        id -> Int4,
        kind -> Varchar,
        descriptor -> Jsonb,
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
    roles (id) {
        id -> Int4,
        name -> Varchar,
        policies -> Jsonb,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        avatar_url -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

joinable!(assignments -> roles (role_id));
joinable!(assignments -> users (user_id));
joinable!(github_accounts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    actions,
    assignments,
    github_accounts,
    roles,
    users,
);
