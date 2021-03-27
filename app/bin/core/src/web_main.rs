extern crate actix_web;
extern crate core_lib as core;

use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {             // TODO default_service 
    return HttpServer::new(|| {         // TODO переместить Scopes в разные методы (https://actix.rs/docs/application/   "Configure")
        App::new().service(             // TODO https://actix.rs/actix-web/actix_web/struct.Scope.html (Service - что это и зачем)
            web::scope("")              // TODO guards для роутов о наличии джвт 
            // .wrap(core::actix_web_component::middleware::scope::test::CheckLogin)
            .route("/user/pre_register", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::pre_register))
            .route("/user/register", web::post().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::register))
            .route("/user/resend_email_for_register", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::resend_email_for_register))
            .route("/user/pre_log_in", web::post().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::pre_log_in))
            .route("/user/resend_email_for_log_in", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::resend_email_for_log_in))
            .route("/user/log_in", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::log_in))
            .route("/user/check_nickname_for_existing", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::check_nickname_for_existing))
            .route("/user/check_email_for_existing", web::get().to(core::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::Authorization::check_email_for_existing))
        )
    }).bind("0.0.0.0:80")?.run().await;
}

// TODO Do not remove this block until the problems have been fixed {
    // TODO -3 Пройтись по всем Unwrap() ( и второму аналогу)
    // TODO -2 Спрятать Апи (Сделать непонятным)
    // TODO -1. Attack types (DDOS, for example)
    // TODO 0. Header Connection: Keep-alive - disable https://developer.mozilla.org/ru/docs/Web/HTTP/%D0%97%D0%B0%D0%B3%D0%BE%D0%BB%D0%BE%D0%B2%D0%BA%D0%B8/Connection
    // TODO 1. Work with JRWT via Redis, create BlacList for Access Token in Redis
    // TODO 2. Diesel do not works with Uuid 0.8.* :
    // https://github.com/diesel-rs/diesel/issues/2348
    // https://github.com/kbknapp/cargo-outdated/issues/216
    // TODO 3. Open/Close connection (Postgres, Redis) or keep connection opened?
    // TODO 4. Work with TimeZone;
    // TODO 5. Update ActixWeb
// Do not remove this block until the problems have been fixed }

// TODO #[inline] - нужно ли оптимизировать с помощью этого атрибута
// TODO разобраться в Cargo.toml (все атрибуты, атрибуты пакетов, ...)
// TODO Проверить запросы в системы (Postgresql, Redis )
// TODO Зaщита от SQL-инъекций
// TODO Разобраться, что именно ФРэймворк параллелит, на каком этапе. (Параллелит ли это файл? - не должен)
// TODO create async Database connections pool (r2d2) - нужно ли. r2d2 держит пул соендинений открытыми и раздает их на каждый хэндлер ( в контексте акстикс веб).
//  То есть, соединения не переоткрываются, а используются постоянные. Сейчас же на каждом воркере будут открыто свое обычное соединение и закрыто.
// TODO дефолтный ответ, если нет роута
// TOOD можно ли изменить деолтный ответ при несовпадении Http параметров с ДТО
// TODO Can we acces to HTTpRequest in Guard? (Check the Params setting opportunity)
// TODO https://www.reddit.com/r/rust/comments/frkta2/manytomany_relationships_in_diesel_does_anybody/      Diesel MANY-TO-MANY Association example