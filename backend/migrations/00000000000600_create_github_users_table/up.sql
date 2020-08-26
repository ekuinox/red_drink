create table github_users (
    github_id integer primary key,
    user_id integer unique not null,
    created_at timestamp not null default current_timestamp,
    foreign key (user_id) references users (id)
)