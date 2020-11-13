insert into roles (
    id, name, policy
) values (
    0, 'admin', '{"allowed":{"resource":["*"],"permissions":["*"]},"denied":{"resource":[],"permissions":[]}}'
)