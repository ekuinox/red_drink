create table github_users (
    github_id integer primary key,
    user_id integer unique not null,
    foreign key (user_id) references users (id)
)