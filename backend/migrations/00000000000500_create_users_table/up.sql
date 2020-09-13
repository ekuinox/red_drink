create table users (
    id serial primary key,
    name varchar unique not null,
    avatar_url varchar,
    email varchar,
    created_at timestamp not null default current_timestamp
)