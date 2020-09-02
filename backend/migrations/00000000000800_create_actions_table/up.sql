create table actions (
    id integer primary key,
    kind varchar not null,
    descriptor json not null,
    created_at timestamp not null default current_timestamp
)