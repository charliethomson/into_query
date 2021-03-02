use super::schema::*;
use into_query::IntoQuery;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
}

#[derive(Deserialize, Queryable, Default, Debug, IntoQuery)]
#[table_name = "users"]
pub struct UserFilter {
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub display_name: Option<String>,
}
