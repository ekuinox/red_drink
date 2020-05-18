table! {
    permissions (path) {
        path -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
