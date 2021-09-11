use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api/v1").service(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    use actix_web::{http, test};

    use super::*;

    #[actix_rt::test]
    async fn test_hello() {
        let resp = hello().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
