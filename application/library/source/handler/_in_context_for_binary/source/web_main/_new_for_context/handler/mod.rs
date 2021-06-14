use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::actix_web_component::middleware::authentication_resolver::authentication_resolver_factory::AuthenticationResolverFactory;
use crate::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::Authorization;
use crate::error::main_error_kind::core::logic_error::LogicError;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::environment_variable_resolver::EnvironmentVariableResolver;
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
use std::path::Path;
use std::path::PathBuf;


pub struct Handler;

impl Handler {
    pub async fn handle() -> Result<(), Error> {
        Self::load_and_check_environment_variables()?;
        Self::configure_log()?;
        Self::run_http_server().await?;

        return Ok(());
    }

    fn load_and_check_environment_variables() -> Result<(), Error> {
        // TODO Ближе к релизу разобраться, как лучше работать с файлами. (То есть, как использовать относительный(relative) пути.)
        // TODO !!!!!!!!!!! Разобраться с путями для ЛООГГЕРА  !!!!!!!!!!!!!!!!
        // TODO Переписать содержимое метода в контексте нахождения директории для .env --------------------------------------------- 
        let file_path_buffer: PathBuf = env::current_exe()?.parent().unwrap().join(&Path::new("../../library")); // TODO не нравится способ взятия и нахождения директории

        let production_environment_file_path_buffer: PathBuf = file_path_buffer.join(&Path::new("prod.env"));
        if production_environment_file_path_buffer.exists() {
            if let Err(error) = dotenv::from_path(production_environment_file_path_buffer.as_path()) {
                return Err(Error::new(ErrorKind::Other, error));
            }

            env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_TRUE)
        } else {
            let development_local_environment_file_path_buffer: PathBuf = file_path_buffer.join(&Path::new("dev.local.env"));
            if development_local_environment_file_path_buffer.exists() {
                if let Err(error) = dotenv::from_path(development_local_environment_file_path_buffer.as_path()) {
                    return Err(Error::new(ErrorKind::Other, error));
                }
            } else {
                let development_environment_file_path_buffer: PathBuf = file_path_buffer.join(&Path::new("dev.env"));
                if development_environment_file_path_buffer.exists() {
                    if let Err(error) = dotenv::from_path(development_environment_file_path_buffer.as_path()) {
                        return Err(Error::new(ErrorKind::Other, error));
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, MainErrorKind::LogicError(LogicError::new("Any ....env file must exist"))));
                }
            }

            env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_FALSE);
        }

        Self::simple_check_environment_variables()?;

        return Ok(());
    }

    fn simple_check_environment_variables() -> Result<(), Error> {
        Self::simple_check_environment_variable(EnvironmentVariableResolver::IS_PRODUCTION_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::SERVER_SOCKET_BINDING_ADDRESS_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::LOGGER_ROLLER_LOG_FILE_NAME_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::LOGGER_LOG_FILE_NAME_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::LOGGER_ENCODER_PATTERN_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::RESOURCE_POSTGRESQL_URL_KEY)?;
        Self::simple_check_environment_variable(EnvironmentVariableResolver::RESOURCE_REDIS_URL_KEY)?;
        
        return Ok(());
    }


    fn simple_check_environment_variable<'outer_a>(environment_variable_key: &'outer_a str) -> Result<(), Error> {
        match env::var(environment_variable_key) {
            Ok(value) => {
                if value.is_empty() {
                    return Err(Error::new(ErrorKind::Other, (environment_variable_key.to_string() + " can not be empty").as_str()));
                }
            },
            Err(var_error) => {
                return Err(Error::new(ErrorKind::Other, format!("{}: {}", environment_variable_key, var_error)));
            }
        }

        return Ok(());
    }

    fn configure_log() -> Result<(), Error> {
        match FixedWindowRoller::builder().base(1).build(EnvironmentVariableResolver::get_logger_roller_log_file_name().as_str(), 10) {
            Ok(fixed_window_roller) => {
                let rolling_file_appender: RollingFileAppender = RollingFileAppender::builder()
                    .append(true)
                    .encoder(Box::new(PatternEncoder::new(EnvironmentVariableResolver::get_logger_encoder_pattern().as_str())))
                    .build(
                        EnvironmentVariableResolver::get_logger_log_file_name(),
                        Box::new(CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024 * 1024)), Box::new(fixed_window_roller)))
                    )?;

                let rolling_file_appender_name: &'static str = "rfa";

                let appender: Appender = Appender::builder().build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));

                let root: Root = Root::builder().appender(rolling_file_appender_name.to_string()).build(LevelFilter::Trace);

                match Config::builder().appender(appender).build(root) {
                    Ok(config) => {
                        if let Err(set_logger_error) = log4rs::init_config(config) {
                            return Err(Error::new(ErrorKind::Other, set_logger_error));
                        }

                        return Ok(());
                    },
                    Err(config_errors) => {
                        return Err(Error::new(ErrorKind::Other, config_errors));
                    }
                }
            },
            Err(error) => {
                return Err(Error::new(ErrorKind::Other, error));
            }
        }
    }

    async fn run_http_server() -> Result<(), Error> {
        let aggregate_connection_pool: AggregateConnectionPool = Self::create_aggregate_connection_pool()?;

        HttpServer::new(move || {
            return App::new()
            .data::<AggregateConnectionPool>(aggregate_connection_pool.clone())
            .configure(Self::configure_http_server);
        })
        .bind(EnvironmentVariableResolver::get_server_socket_binding_address())?
        .run()
        .await?;

        return Ok(());
    }

    fn create_aggregate_connection_pool() -> Result<AggregateConnectionPool, Error> {
        match AggregateConnectionPool::new() {
            Ok(aggregate_connection_pool) => {
                return Ok(aggregate_connection_pool);
            },
            Err(resource_error_kind) => {
                return Err(Error::new(ErrorKind::Other, resource_error_kind));
            }
        }
    }

    fn configure_http_server<'outer_a>(service_config: &'outer_a mut ServiceConfig) -> () {
        service_config     // TODO default_service 
        .service(
            web::scope("/api")
            .service(
                web::scope("/v1")
                .service(
                    web::scope("/m")
                    .service(
                        web::scope("/na")    // TODO NotAuthorized  // TODO можно ли объединить в Скопе без роута. ( Если есть Скоуп с пустым Роуом, то другие Скоцпы не воспринимаютмя)
                        .service( 
                            web::scope("/au")
                            .route("/pr", web::post().to(Authorization::pre_register))
                            .route("/r", web::post().to(Authorization::register))
                            .route("/refr", web::post().to(Authorization::resend_email_for_register))
                            .route("/pli", web::post().to(Authorization::pre_log_in))
                            .route("/refl", web::post().to(Authorization::resend_email_for_log_in))
                            .route("/li", web::post().to(Authorization::log_in))
                            .route("/cnfe", web::get().to(Authorization::check_nickname_for_existing))
                            .route("/cefe", web::get().to(Authorization::check_email_for_existing))
                            .route("/rjawt", web::post().to(Authorization::refresh_json_access_web_token))
                            .route("/prp", web::post().to(Authorization::pre_reset_password))
                            .route("/rp", web::post().to(Authorization::reset_password))
                            .route("/refrp", web::post().to(Authorization::resend_email_for_reset_password))
                        )
                    )
                    .service(
                        web::scope("/a")     // TODO Authorized
                        .wrap(AuthenticationResolverFactory)
                        .service( 
                            web::scope("/au")
                            .route("/lo", web::post().to(Authorization::log_out))
                            .route("/lofad", web::post().to(Authorization::log_out_from_all_devices))
                        )
                    )
                )
            )
        );

        return ();
    }
}