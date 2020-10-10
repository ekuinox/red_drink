create table roles (
    id serial primary key,
    name varchar not null unique,
    accessible jsonb not null,
    created_at timestamp not null default current_timestamp
)