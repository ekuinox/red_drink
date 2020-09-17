create table permissions (
    path varchar primary key,
    name varchar not null,
    description varchar,
    created_at timestamp not null default current_timestamp
)