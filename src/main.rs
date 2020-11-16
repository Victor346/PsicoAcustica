mod schema;
mod models;
mod actions;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, post, middleware, App, HttpServer, HttpResponse, Result, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::embed_migrations;
use std::collections::HashMap;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}
#[get("/game")]
async fn game() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/game.html")?)
}

#[get("/favicon.ico")]
async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

#[get("/api/comments/{page}")]
async fn paged_comments(pool: web::Data<DbPool>, page: web::Path<i64>)
    -> HttpResponse {
    let page_number = page.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let comments = web::block(move || actions::get_paged_comments(&conn, page_number))
        .await.unwrap();

    let conn = pool.get().expect("couldn't get connection from pool");

    let total_count = web::block(move || actions::get_count_comments(&conn))
        .await.unwrap();

    let result = models::CommentsListResult {
        comments,
        total: total_count,
    };

    HttpResponse::Ok().json(result)
}

#[post("/api/comment")]
async fn create_comment(pool: web::Data<DbPool>, comment_json: web::Json<models::NewComment>)
    -> HttpResponse {
    let new_comment = comment_json.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let inserted_id = web::block(move || actions::insert_comment(&conn, new_comment))
        .await.unwrap();

    let mut result = HashMap::new();
    result.insert("id", inserted_id);
    HttpResponse::Created().json(result)
}



embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // Database Setup
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Migration setups

    let conn = pool.get().expect("couldn't get connection from pool");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Error in migrations");

    // Http Server Setup

    let address = format!("0.0.0.0:{}",match std::env::var("PORT") {
        Ok(p) => p,
        Err(_e) => "8080".to_string(),
    });

    println!("Serving at: {}", address);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(game)
            .service(fs::Files::new("/game", "./static/game"))
            .service(hello)
            .service(favicon)
            .service(paged_comments)
            .service(create_comment)
    })
        .bind(address)?
        .run()
        .await
}
