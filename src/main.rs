use actix_web::{App, HttpServer, Result, get};
use actix_files::NamedFile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(default_style)
            .service(home)
    })
    //.bind("0.0.0.0", 8080)?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/default.css")]
async fn default_style() -> Result<NamedFile> {
    let path = "./static/default.css";
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn home() -> Result<NamedFile> {
    let path = "./static/home.html";
    Ok(NamedFile::open(path)?)
}
