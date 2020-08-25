create table resources (
    id varchar not null,
    name varchar not null,
    description varchar not null default '',
    created_at timestamp not null default current_timestamp,
    primary key(id)
)