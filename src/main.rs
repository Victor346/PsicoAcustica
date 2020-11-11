use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Result};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(game)
            .service(fs::Files::new("/game", "./static/game"))
            .service(hello)
            .service(favicon)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
