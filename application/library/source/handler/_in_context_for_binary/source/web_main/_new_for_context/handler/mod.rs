use actix_web::App;
use actix_web::HttpServer;
use crate::actix_web_component::configuration::main_service_configurator::MainServiceConfigurator;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use dotenv;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config:: Root;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::encode::pattern::PatternEncoder;
use std::env;
use std::io::Error;
use std::io::ErrorKind;

pub struct Handler;

impl Handler {// TODO сделать реестр констант - енв значений. // TODO доделать на env:: , 
    pub async fn handle() -> Result<(), Error> {
        Self::load_environment_variables()?;
        Self::configure_log()?;

        let aggregate_connection_pool: AggregateConnectionPool = Self::create_aggregate_connection_pool()?;

        return HttpServer::new(move || {
            return App::new()
            .data::<AggregateConnectionPool>(aggregate_connection_pool.clone())
            .configure(MainServiceConfigurator::configure); //TODO Вынести конфигуратор сюда
        })
        .bind(env::var("SERVER_SOCKET_BINDING_ADDRESS").unwrap())?
        .run()
        .await;
    }

    fn configure_log() -> Result<(), Error> {
        match FixedWindowRoller::builder().base(1).build(env::var("LOGGER_ROLLER_LOG_FILE_NAME").unwrap().as_str(), 10) {
            Ok(fixed_window_roller) => {
                let rolling_file_appender: RollingFileAppender = RollingFileAppender::builder()
                    .append(true)
                    .encoder(Box::new(PatternEncoder::new(env::var("LOGGER_ENCODER_PATTERN").unwrap().as_str())))
                    .build(
                        env::var("LOGGER_LOG_FILE_NAME").unwrap(),
                        Box::new(CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024 * 1024)), Box::new(fixed_window_roller)))
                    )?;

                let rolling_file_appender_name: &'static str = "rolling_file_appender";

                let appender: Appender = Appender::builder().build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));

                let root: Root = Root::builder().appender(rolling_file_appender_name.to_string()).build(LevelFilter::Trace);

                match Config::builder().appender(appender).build(root) {
                    Ok(config) => {
                        if let Err(set_logger_error) = log4rs::init_config(config) {
                            return Err(Error::new::<String>(ErrorKind::Other, set_logger_error.to_string()));
                        }

                        return Ok(());
                    },
                    Err(config_errors) => {
                        return Err(Error::new::<String>(ErrorKind::Other, config_errors.to_string()));
                    }
                }
            },
            Err(error) => { // TODO понять, включен ли бэктрейс. Если включен, отключить.
                return Err(Error::new::<String>(ErrorKind::Other, error.to_string()));
            }
        }
    }

    fn create_aggregate_connection_pool() -> Result<AggregateConnectionPool, Error> {
        match AggregateConnectionPool::new() {
            Ok(aggregate_connection_pool) => {
                return Ok(aggregate_connection_pool);
            },
            Err(resource_error_kind) => {
                return Err(Error::new::<String>(ErrorKind::Other, format!("{}", resource_error_kind)));
            }
        }
    }

    fn load_environment_variables() -> Result<(), Error> {
        // TODO Ближе к релизу разобраться, как лучше работать с файлами. (То есть, как использовать относительный(relative) пути.)
        // TODO Понять, почему в Оперативную память загружаются параметры-не-только-значения в .env (то есть, лишние системные параметры, например, от VSCode)
        // TODO !!!!!!!!!!! Разобраться с путями для ЛООГГЕРА  !!!!!!!!!!!!!!!!
        // TODO Переписать содержимое метода ---------------------------------------------
        dotenv::from_path(env::current_exe()?.parent().unwrap().to_str().unwrap().to_string() + "/../../library/.env").unwrap();
        // TODO Переписать содержимое метода ---------------------------------------------

        return Ok(());
    }
}