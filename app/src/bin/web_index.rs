use std::io::Result;

#[macro_use]
extern crate actix_web;
use actix_web::App;
use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> Result<()> {
    return HttpServer::new(|| {
        App::new().service(test)// TODO delete service
    }).bind("0.0.0.0:80")?.run().await;
}

// TODO create async Database connections pool
// TODO default_service 
// TODO Middleware for Scope/Resource
// TODO Custom Handlers 

// TODO Can we acces to HTTpRequest in Guard? (Check the Params setting opportunity)









// TODO delete--------------------------------------------------------
use actix_web::Responder;
use actix_web::HttpResponse;
#[get("/test")]
async fn test() -> impl Responder {
    return HttpResponse::Ok().body("test");
}
// TODO delete---------------------------------------------------------