#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::{User, NewUser, UpdateUser};
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

// Create user (Admin only)
async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let new_user = new_user.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })?;
    Ok(HttpResponse::Ok().json(inserted_user))
}

// Update user (Admin only)
async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    update: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let updated_user: User = diesel::update(users.filter(id.eq(*user_id)))
        .set(&update.into_inner())
        .get_result(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(updated_user))
}

// Deactivate user (Admin only)
async fn deactivate_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let updated_user: User = diesel::update(users.filter(id.eq(*user_id)))
        .set(active.eq(false))
        .get_result(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(updated_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}/deactivate", web::put().to(deactivate_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
