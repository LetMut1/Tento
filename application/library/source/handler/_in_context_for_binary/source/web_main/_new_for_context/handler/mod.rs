use actix_web::App;
use actix_web::HttpServer;
use crate::actix_web_component::configuration::main_service_configurator::MainServiceConfigurator;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config:: Root;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::encode::pattern::PatternEncoder;
use std::io::Error;
use std::io::ErrorKind;

pub struct Handler;

impl Handler {
    pub async fn handle() -> Result<(), Error> {
        Self::configure_log()?;

        let aggregate_connection_pool: AggregateConnectionPool = Self::create_aggregate_connection_pool()?;

        return HttpServer::new(move || {
            return App::new()
            .data::<AggregateConnectionPool>(aggregate_connection_pool.clone())
            .configure(MainServiceConfigurator::configure); //TODO СТоит ли КОнфигуратор вынести сюда в етод хендлера
        })
        .bind("0.0.0.0:80")?.run().await; // TODO env or method 
    }

    fn configure_log() -> Result<(), Error> {
        match FixedWindowRoller::builder().base(1).build("/log/common-past-{}.log", 20) {   // TODO from env
            Ok(fixed_window_roller) => {
                let rolling_file_appender: RollingFileAppender = RollingFileAppender::builder()
                    .append(true)
                    .encoder(Box::new(PatternEncoder::new("[{d}] [{l}]: {m}{n}")))    // TODO from env
                    .build(
                        "/log/common-current.log",  // TODO from env
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
            Err(error) => {                                 // TODO понять, включен ли бэктрейс. Если включен, отключить.
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
}