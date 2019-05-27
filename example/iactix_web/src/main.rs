use actix_web::{web, App, HttpServer, Responder,HttpResponse};
fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}
const STR: &str = "Hello World";
fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(
              web::resource("/").route(web::to(|| HttpResponse::Ok().body(STR)))))
        .bind("127.0.0.1:3000")?
        .run()
}
