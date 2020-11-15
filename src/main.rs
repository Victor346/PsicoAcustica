use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Result};

#[get("/")]
async fn hello() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(game)
            .service(fs::Files::new("/game", "./static/game"))
            .service(hello)
            .service(credits)
            .service(info01)
            .service(info02)
            .service(info03)
            .service(info04)
            .service(favicon)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
