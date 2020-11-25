mod schema;
mod models;
mod actions;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::models::Comment;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, post, middleware, App, HttpServer, HttpResponse, Result, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::embed_migrations;
use std::collections::HashMap;
use askama::Template;
use chrono::Utc;

#[derive(Template)]
#[template(path = "comments.html")]
struct CommentsTemplate<> {
    total: i64,
    current_page: i64,
    comments: Vec<Comment>,
    is_end: bool,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> Result<NamedFile> {
    if Utc::now().timestamp() < 1606543200 {
        Ok(NamedFile::open("./static/progress.html")?)
    } else {
        Ok(NamedFile::open("./static/index.html")?)
    }
}

#[get("/credits")]
async fn credits() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/credits.html")?)
}

#[get("/info01")]
async fn info01() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/infographics/info01.html")?)
}
#[get("/info02")]
async fn info02() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/infographics/info02.html")?)
}
#[get("/info03")]
async fn info03() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/infographics/info03.html")?)
}
#[get("/info04")]
async fn info04() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/infographics/info04.html")?)
}

#[get("/game")]
async fn game() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/game.html")?)
}


#[get("/favicon.ico")]
async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

#[get("/comments/{page}")]
async fn comments_page(pool: web::Data<DbPool>, page: web::Path<i64>) -> HttpResponse {
    let page_number = page.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let comments = web::block(move || actions::get_paged_comments(&conn, page_number))
        .await.unwrap();

    let conn = pool.get().expect("couldn't get connection from pool");

    let total_count = web::block(move || actions::get_count_comments(&conn))
        .await.unwrap();

    let page_amount = (total_count as f32 / 10.0).ceil();

    let is_end = page_amount == (page_number as f32);

    let s = CommentsTemplate {
        total: total_count,
        current_page: page_number,
        comments,
        is_end,
    }
        .render()
        .unwrap();

    HttpResponse::Ok().content_type("text/html").body(s)
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

#[post("/comments/1")]
async fn create_comment_form(pool: web::Data<DbPool>, comment_json: web::Form<models::NewComment>)
                        -> HttpResponse {
    let new_comment = comment_json.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let inserted_id = web::block(move || actions::insert_comment(&conn, new_comment))
        .await.unwrap();


    let page_number = 1;
    let conn = pool.get().expect("couldn't get connection from pool");

    let comments = web::block(move || actions::get_paged_comments(&conn, page_number))
        .await.unwrap();

    let conn = pool.get().expect("couldn't get connection from pool");

    let total_count = web::block(move || actions::get_count_comments(&conn))
        .await.unwrap();

    let page_amount = (total_count as f32 / 10.0).ceil();

    let is_end = page_amount == (page_number as f32);

    let s = CommentsTemplate {
        total: total_count,
        current_page: page_number,
        comments,
        is_end,
    }
        .render()
        .unwrap();

    HttpResponse::Ok().content_type("text/html").body(s)
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
            .service(fs::Files::new("/static/imgs", "./static/imgs"))
            .service(hello)
            .service(credits)
            .service(info01)
            .service(info02)
            .service(info03)
            .service(info04)
            .service(favicon)
            .service(paged_comments)
            .service(create_comment)
            .service(comments_page)
            .service(create_comment_form)
    })
        .bind(address)?
        .run()
        .await
}
