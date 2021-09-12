use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

mod messages;

#[get("/hello")]
async fn hello() -> impl Responder {
    let domain_msg = domain::generate_message();
    let msg = messages::Message {
        title: domain_msg.title,
        body: domain_msg.body,
    };
    HttpResponse::Ok().json(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api/v1").service(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
