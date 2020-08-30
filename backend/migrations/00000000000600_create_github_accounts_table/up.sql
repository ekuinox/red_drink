create table github_accounts (
    github_id integer primary key,
    user_id integer unique not null,
    created_at timestamp not null default current_timestamp,
    foreign key (user_id) references users (id)
)