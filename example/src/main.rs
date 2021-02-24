use std::env;

use diesel::{prelude::*, Connection, OptionalExtension, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use into_query::IntoQuery;
use models::User;

#[macro_use]
extern crate diesel;

mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_user() -> Option<models::User> {
    use crate::schema::users::dsl::*;
    let conn = establish_connection();

    let new_user = models::NewUser {
        username: "Testuser".into(),
        display_name: "Test user".into(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(&conn)
        .optional()
        .expect("Failed")
}

fn find_user(name: String) -> QueryResult<Vec<models::User>> {
    let filter = models::FindUser {
        username: Some(name),
        ..models::FindUser::default()
    };

    let query = filter.into_query();
    let query_str = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    println!("Built query: '{}'", query_str.to_string());

    return query.get_results::<User>(&establish_connection());
}

fn main() {
    println!("Inserting test data...");
    println!("Result: {:#?}", create_user());
    println!("Looking for user whose usernames are Testuser (test data)...");
    println!("{:#?}", find_user("Testuser".into()));
}
