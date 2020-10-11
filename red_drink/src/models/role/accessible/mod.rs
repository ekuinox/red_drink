use diesel::sql_types::Jsonb;

mod accessible_impl;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Allowed {
    resources: Vec<String>,
    permissions: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Denied {
    resources: Vec<String>,
    permissions: Vec<String>
}

#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Default, Clone)]
#[sql_type = "Jsonb"]
pub struct Accessible {
    allowed: Allowed,
    denied: Denied
}
