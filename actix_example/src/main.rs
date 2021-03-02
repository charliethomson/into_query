use std::env;

use actix_web::{
    get, post,
    web::{Json, Query},
    App, HttpResponse, HttpServer,
};
use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use into_query::IntoQuery;
use models::User;
use schema::users::dsl::*;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate diesel;

mod models;
mod schema;

#[derive(Serialize, Deserialize)]
pub struct OkMessage<Message> {
    pub ok: bool,
    pub message: Option<Message>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[post("/api/users")]
pub async fn users_post(Json(body): Json<models::NewUser>) -> HttpResponse {
    let conn = establish_connection();
    match diesel::insert_into(users).values(body).execute(&conn) {
        Ok(_) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some("Inserted User".to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[get("/api/users")]
pub async fn users_get(Query(filter): Query<models::UserFilter>) -> HttpResponse {
    let query = filter.into_query();
    match query.get_results::<User>(&establish_connection()) {
        Ok(results) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some(results.into_iter().collect::<Vec<models::User>>()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

const URL: &str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server =
        HttpServer::new(move || App::new().service(users_post).service(users_get)).bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
