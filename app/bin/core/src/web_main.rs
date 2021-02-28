extern crate actix_web;
extern crate core_lib as core;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use std::io::Result;      // TODO Везде ли Нужен МэйбиОвнед? при создании экземпляра в архументы можно положить сразу референс. - То есть, в основном, будут референсы

#[actix_web::main]
async fn main() -> Result<()> {
    return HttpServer::new(|| {         // TODO переместить Scops в разные методы? 
        App::new().service(
            web::scope("")              // TODO сделать правильно
            .route("/user/register", web::post().to(core::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::Authorization::register))
            .route("/user/cnfe", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::Authorization::check_nickname_for_existing))
            .route("/user/log_in", web::post().to(core::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::Authorization::log_in))
        )
    }).bind("0.0.0.0:80")?.run().await;
}

// TODO Do not remove this block until the problems have been fixed {
    // TOOD 0. Header Connection: Keep-alive - disable https://developer.mozilla.org/ru/docs/Web/HTTP/%D0%97%D0%B0%D0%B3%D0%BE%D0%BB%D0%BE%D0%B2%D0%BA%D0%B8/Connection
    // TODO 1. Work with JRWT via Redis, create BlacList for Access Token in Redis
    // TODO 2. Diesel do not works with Uuid 0.8.* :
    // https://github.com/diesel-rs/diesel/issues/2348
    // https://github.com/kbknapp/cargo-outdated/issues/216
    // TODO 3. Open/Close connection (Postgres, Redis) or keep connection opened?
    // TODO 4. Work with TimeZone;
    // TODO 5. Update ActixWeb
// Do not remove this block until the problems have been fixed }

// TODO Зaщита от SQL-инъекций
// TODO Разобраться, что именно ФРэймворк параллелит, на каком этапе. (Параллелит ли это файл? - не должен)
// TODO create async Database connections pool (r2d2) - нужно ли. r2d2 держит пул соендинений открытыми и раздает их на каждый хэндлер ( в контексте акстикс веб).
//  То есть, соединения не переоткрываются, а используются постоянные. Сейчас же на каждом воркере будут открыто свое обычное соединение и закрыто.
// TODO default_service 
// TODO дефолтный ответ, если нет роута
// TOOD можно ли изменить деолтный ответ при несовпадении Http параметров с ДТО
// TODO Middleware for Scope/Resource
// TODO Can we acces to HTTpRequest in Guard? (Check the Params setting opportunity)
// TODO https://www.reddit.com/r/rust/comments/frkta2/manytomany_relationships_in_diesel_does_anybody/      Diesel MANY-TO-MANY Association example