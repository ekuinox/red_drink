create table actions (
    id serial primary key,
    kind varchar not null,
    descriptor varchar not null,
    created_at timestamp not null default current_timestamp
)