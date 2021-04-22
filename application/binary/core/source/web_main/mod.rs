extern crate actix_web;
extern crate core_library;

use actix_web::App;
use actix_web::HttpServer;
use core_library::actix_web_component::configuration::main_service_configurator::MainServiceConfigurator;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()>
{
    return HttpServer::new(|| {
        return App::new().configure(MainServiceConfigurator::configure);
    }).bind("0.0.0.0:80")?.run().await; // TODO адрес через метод брать, 
}

// TODO Do not remove this block until the problems have been fixed {
    // TODO -6 Разобраться в том, как Правильно делать процессы в Редис Атомарными ( аналогия транзакции). Например, безопасное удаление.
    // TODO -5 более короткоживужие JAWT для админов разного рода
    // TODO -4 Пройтись по всем Unwrap() ( и второму аналогу)
    // TODO -3 Query deserialize error: missing field `n` ( Убрать дефотлный ответ) Послать неправильные запросы, (пустой параметр, Нулл - поэкспериментировать)
    // TODO -2 Спрятать Апи (Сделать непонятным)
    // TODO -1. Attack types (DDOS, for example)
    // TODO 0. Header Connection: Keep-alive - disable https://developer.mozilla.org/ru/docs/Web/HTTP/%D0%97%D0%B0%D0%B3%D0%BE%D0%BB%D0%BE%D0%B2%D0%BA%D0%B8/Connection
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
// То есть, соединения не переоткрываются, а используются постоянные. Сейчас же на каждом экшене будут открыто свое обычное соединение и закрыто.
// Нужно ли делать пулл ( имеется ввиду, постоянное соединение ( для Бд постоянно открываем-закрываем)) для Редиса. Скорость устанровлния больше скорости обработки
// TODO дефолтный ответ, если нет роута
// TODO Заменить все дефолтные ответы. (например, неправильный Content-Type)
// TODO можно ли изменить деолтный ответ при несовпадении Http параметров с ДТО
// TODO Can we acces to HTTpRequest in Guard? (Check the Params setting opportunity)
// TODO https://www.reddit.com/r/rust/comments/frkta2/manytomany_relationships_in_diesel_does_anybody/      Diesel MANY-TO-MANY Association example