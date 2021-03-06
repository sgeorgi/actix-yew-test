use actix_files as fs;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

mod messages;

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub title: String,
    pub body: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    let domain_msg = messages::generate_message();
    let msg = Message {
        title: domain_msg.title,
        body: domain_msg.body,
    };
    HttpResponse::Ok().json(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").service(hello))
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
        .bind("0.0.0.0:9000")?
        .run()
        .await
}
