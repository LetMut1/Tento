extern crate application;

use application::application_layer::service::handler::_in_contex_for::presentation_layer::command::run_server::_new_for_context::base::Base as RunServerHandler;

fn main(
) -> () {
    if let Err(base_error) = RunServerHandler::handle(std::file!().to_string()) {
        println!("{}", base_error);
    }

    return ();
}

// TODO Ограничивать количество запросов с одного адреса на специфические урлы. (Например, отправка писем, и так далее)
// TODO Ограничить максимальный развмер данных во входящем запросе на апи. (Проверять длину Хедеров, проверять длину Тела)
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
// TODO #[inline] - нужно ли оптимизировать с помощью этого атрибута