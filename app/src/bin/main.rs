use std::io::Result;

#[macro_use]
extern crate actix_web;
use actix_web::App;
use actix_web::HttpServer;


// TODO delete--------------------------------------------------------
use actix_web::Responder;
use actix_web::HttpResponse;
#[get("/test")]
async fn test() -> impl Responder {
    return HttpResponse::Ok().body("test");
}
// TODO delete---------------------------------------------------------


#[actix_web::main]
async fn main() -> Result<()> {
    return HttpServer::new(|| {
        App::new().service(test)                          // delete service
    }).bind("0.0.0.0:80")?.run().await;
}