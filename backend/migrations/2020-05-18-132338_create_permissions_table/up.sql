create table permissions (
    path varchar primary key,
    name varchar not null,
    description varchar,
    created_at TIMESTAMP not null
)