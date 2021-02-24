use super::schema::*;
use into_query::IntoQuery;
#[derive(Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
}

#[derive(IntoQuery, Default)]
#[table_name = "users"]
pub struct FindUser {
    pub username: Option<String>,
    pub display_name: Option<String>,
}
