create table users (
    id serial primary key,
    created_at timestamp not null default current_timestamp
)