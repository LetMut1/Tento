#[macro_use]
extern crate actix_web;
use actix_web::App;
use actix_web::HttpServer;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    return HttpServer::new(|| {
        App::new().service(test)// TODO delete service
    }).bind("0.0.0.0:80")?.run().await;
}

// TODO Do not remove this block until the problems have been fixed {
    // TODO 1. Diesel do not works with Uuid 0.8.* :
    // https://github.com/diesel-rs/diesel/issues/2348
    // https://github.com/kbknapp/cargo-outdated/issues/216
// Do not remove this block until the problems have been fixed }


// TODO create async Database connections pool
// TODO Как Закрывать connection к БД
// TODO зачем Dereference на Self в ToSql (Diesel) - !"Это не дереф, это Клонирование"!
// TODO default_service 
// TODO Middleware for Scope/Resource
// TODO Custom Handlers 
// TODO Can we acces to HTTpRequest in Guard? (Check the Params setting opportunity)
// TODO delete--------------------------------------------------------
use actix_web::Responder;
use actix_web::HttpResponse;
#[get("/test")]
async fn test() -> impl Responder {
    // println!("{:?}", value);
    return HttpResponse::Ok().body("dfdfvdf");
}
// TODO delete---------------------------------------------------------