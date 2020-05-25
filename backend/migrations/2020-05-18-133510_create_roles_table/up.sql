create table roles (
    id integer primary key,
    name varchar not null unique,
    created_at timestamp not null default current_timestamp
)