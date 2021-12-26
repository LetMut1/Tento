extern crate actix_web;
extern crate application;

use application::application_layer::service::handler::_in_contex_for::presentation_layer::command::run_server::_new_for_context::base::Base as RunServerHandler;

#[actix_web::main]
async fn main(
) -> () {   // TODO перед релизом понять, имеет ли значение, что именно возвращать в main. Err(...) для Result. Или просто void. (Как понять при деплое, что бинарник верну ошибку)
    if let Err(error) = RunServerHandler::handle().await {
        println!("{}", error);
    }

    return ();
}

// TODO Do not remove this block until the problems have been fixed {
    // TODO Ограничивать количество запросов с одного адреса на специфические урлы. (Например, отправка писем, и так далее)
    // TODO Как ограничить максимальный развмер данных во входящем запросе на апи.
    // TODO При работе с лайками (любой быстро меняющейся сущность) использвать Редис-транзакции (они нужно только для Атомарности, чтобы не было состояния гонки.
    // Вопрос. Испоьзуется оптимистичная блокировка, то есть, один процесс в одно время, второй ждет или отменяется?)
    // TODO Разобраться в том, как Правильно делать процессы в Редис Атомарными ( аналогия транзакции). Например, безопасное удаление.
    // TODO  Более короткоживужие JAWT для админов разного рода
    // TODO  Спрятать Апи (Сделать непонятным)
    // TODO  Attack types (DDOS, for example)
    // TODO  Header Connection: Keep-alive - disable https://developer.mozilla.org/ru/docs/Web/HTTP/%D0%97%D0%B0%D0%B3%D0%BE%D0%BB%D0%BE%D0%B2%D0%BA%D0%B8/Connection
    // TODO  Work with TimeZone;
    // TODO  Update ActixWeb
    // TODO  Изучить конфигурации Рсурсов( Postgresql, Redis, ...)
    // TODO пробрасывать в .env Dev/Test/Prod срреды
    // TODO  Почитать про Cargo и понять, как делать сборку хорошо. Как работать тогда с фалйами, (проблема доступа к Енв файлу)(куда класть скомпилированный файл, чтобы относительно ннего запускать искать етв)
// Do not remove this block until the problems have been fixed }

// TODO #[inline] - нужно ли оптимизировать с помощью этого атрибута
// TODO разобраться в Cargo.toml (все атрибуты, атрибуты пакетов, ...)
// TODO Проверить запросы в системы (Postgresql, Redis )
// TODO Зaщита от SQL-инъекций
// TODO create async Database connections pool (r2d2) - нужно ли. r2d2 держит пул соендинений открытыми и раздает их на каждый хэндлер ( в контексте акстикс веб).
// То есть, соединения не переоткрываются, а используются постоянные. Сейчас же на каждом экшене будут открыто свое обычное соединение и закрыто.
// Нужно ли делать пулл ( имеется ввиду, постоянное соединение ( для Бд постоянно открываем-закрываем)) для Редиса. Скорость устанровлния больше скорости обработки
// TODO дефолтный ответ, если нет роута
// TODO Заменить все дефолтные ответы. (например, неправильный Content-Type)
// TODO можно ли изменить деолтный ответ при несовпадении Http параметров с ДТО
// TODO Can we acces to HTTpRequest in Guard? (Check the Params setting opportunity)