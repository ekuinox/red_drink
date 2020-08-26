create table accessibles (
    role_id integer not null,
    permission_path varchar not null,
    resource_id varchar, /** nullの場合はすべてのリソースが対象として扱う **/
    created_at timestamp not null default current_timestamp,
    primary key(role_id, permission_path),
    foreign key (role_id) references roles (id),
    foreign key (permission_path) references permissions (path),
    foreign key (resource_id) references resources (id)
)