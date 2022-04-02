use super::schema::*;
use into_query::IntoQuery;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Queryable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
}

#[derive(Deserialize, Queryable, Default, Debug, IntoQuery)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct UserFilter {
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub display_name: Option<String>,
}
