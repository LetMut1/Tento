use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crate::presentation_layer::service::actix_web::middleware::authentication_resolver_factory::AuthenticationResolverFactory;
use crate::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization as RequestHandlerApplicationUserAuthorization;
use crate::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::Base as RequestHandlerChannelBase;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use std::env;
use std::path::Path;
use std::path::PathBuf;

pub struct Base;

impl Base {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "production.env";  // TODO Посмотреть, какие есть еще лучшие форматы аналоги .env (Может, Томл?)
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.env";
    const DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME: &'static str = "development.local.env";

    #[actix_web::main] 
    pub async fn handle(
        binary_file_path: String
    ) -> Result<(), BaseError> {
        Self::load_and_check_environment_variables(binary_file_path.as_str())?;
        Self::configure_log()?;
        Self::run_http_server().await?;

        return Ok(());
    }

    fn load_and_check_environment_variables<'a>(
        binary_file_path: &'a str
    ) -> Result<(), BaseError> {
        match Path::new(binary_file_path).parent() {
            Some(file_path) => {
                let production_environment_file_path_buffer: PathBuf = file_path.join(&Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME));
                if production_environment_file_path_buffer.exists() {
                    dotenv::from_path(production_environment_file_path_buffer.as_path())?;

                    env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_TRUE)
                } else {
                    let development_local_environment_file_path_buffer: PathBuf = file_path.join(&Path::new(Self::DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME));
                    if development_local_environment_file_path_buffer.exists() {
                        dotenv::from_path(development_local_environment_file_path_buffer.as_path())?;
                    } else {
                        let development_environment_file_path_buffer: PathBuf = file_path.join(&Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME));
                        if development_environment_file_path_buffer.exists() {
                            dotenv::from_path(development_environment_file_path_buffer.as_path())?;
                        } else {
                            return Err(BaseError::LogicError {logic_error: LogicError::new(false, "Any ....env files does not exist.")});
                        }
                    }

                    env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_FALSE);
                }

                Self::check_environment_variables()?;

                return Ok(());
            },
            None => {
                return Err(BaseError::LogicError {logic_error: LogicError::new(false, "The directory does not exist.")});
            }
        }
    }

    fn check_environment_variables(
    ) -> Result<(), BaseError> {
        EnvironmentVariableResolver::is_production()?;
        EnvironmentVariableResolver::get_server_socket_address()?;
        EnvironmentVariableResolver::get_logger_roller_log_file_name()?;
        EnvironmentVariableResolver::get_logger_log_file_name()?;
        EnvironmentVariableResolver::get_logger_encoder_pattern()?;
        EnvironmentVariableResolver::get_security_jrwt_encoding_private_key()?;
        EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_key()?;
        EnvironmentVariableResolver::get_resource_postgresql_url()?;
        EnvironmentVariableResolver::get_resource_redis_url()?;
        EnvironmentVariableResolver::get_resource_email_server_socket_address()?;

        return Ok(());
    }

    fn configure_log(
    ) -> Result<(), BaseError> {
        let fixed_window_roller: FixedWindowRoller = FixedWindowRoller::builder()
            .base(1)
            .build(EnvironmentVariableResolver::get_logger_roller_log_file_name()?.as_str(), 10)?;

        let rolling_file_appender: RollingFileAppender = RollingFileAppender::builder()
            .append(true)
            .encoder(Box::new(PatternEncoder::new(EnvironmentVariableResolver::get_logger_encoder_pattern()?.as_str())))
            .build(
                EnvironmentVariableResolver::get_logger_log_file_name()?,
                Box::new(CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024 * 1024)), Box::new(fixed_window_roller)))
            )?;

        let rolling_file_appender_name: &'static str = "rfa";

        let appender: Appender = Appender::builder().build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));

        let root: Root = Root::builder().appender(rolling_file_appender_name.to_string()).build(LevelFilter::Trace);

        let config: Config = Config::builder().appender(appender).build(root)?;

        log4rs::init_config(config)?;

        return Ok(());
    }

    async fn run_http_server(
    ) -> Result<(), BaseError> {
        let aggregate_connection_pool: AggregateConnectionPool = AggregateConnectionPool::new()?;

        HttpServer::new(
            move || {
                return App::new()
                .data::<AggregateConnectionPool>(aggregate_connection_pool.clone())
                .configure(Self::configure_http_server);
            }
        )
        .bind(EnvironmentVariableResolver::get_server_socket_address()?)?
        .run()
        .await?;

        return Ok(());
    }

    fn configure_http_server<'a>(
        service_config: &'a mut ServiceConfig
    ) -> () {
        service_config     // TODO default_service 
        .service(
            web::scope("/v1")
            .service(
                web::scope("/m")
                .service(
                    web::scope("/na")   // TODO NotAuthorized. Можно ли в новой версии АкстикаВеба убрать этоу чать пути 
                    .service( 
                        web::scope("/au")
                        .route("/cnfe", web::get().to(RequestHandlerApplicationUserAuthorization::check_nickname_for_existing))
                        .route("/cefe", web::get().to(RequestHandlerApplicationUserAuthorization::check_email_for_existing))
                        .route("/rbfs", web::post().to(RequestHandlerApplicationUserAuthorization::register_by_first_step))
                        .route("/rbls", web::post().to(RequestHandlerApplicationUserAuthorization::register_by_last_step))
                        .route("/sefr", web::post().to(RequestHandlerApplicationUserAuthorization::send_email_for_register))
                        .route("/libfs", web::post().to(RequestHandlerApplicationUserAuthorization::log_in_by_first_step))
                        .route("/libls", web::post().to(RequestHandlerApplicationUserAuthorization::log_in_by_last_step))
                        .route("/sefli", web::post().to(RequestHandlerApplicationUserAuthorization::send_email_for_log_in))
                        .route("/rpbfs", web::post().to(RequestHandlerApplicationUserAuthorization::reset_password_by_first_step))
                        .route("/rpbls", web::post().to(RequestHandlerApplicationUserAuthorization::reset_password_by_last_step))
                        .route("/sefrp", web::post().to(RequestHandlerApplicationUserAuthorization::send_email_for_reset_password))
                        .route("/rjawt", web::post().to(RequestHandlerApplicationUserAuthorization::refresh_json_access_web_token))
                    )
                )
                .service(
                    web::scope("/a")
                    .wrap(AuthenticationResolverFactory)
                    .service( 
                        web::scope("/au")
                        .route("/lo", web::post().to(RequestHandlerApplicationUserAuthorization::log_out))
                        .route("/lofad", web::post().to(RequestHandlerApplicationUserAuthorization::log_out_from_all_devices))
                    )
                    .service( 
                        web::scope("/c")
                        .route("/gmbn", web::get().to(RequestHandlerChannelBase::get_many_by_name))
                        .route("/gmbca", web::get().to(RequestHandlerChannelBase::get_many_by_created_at))
                        .route("/gmbsq", web::get().to(RequestHandlerChannelBase::get_many_by_subscribers_quantity))
                        .route("/gmbir", web::get().to(RequestHandlerChannelBase::get_many_by_id_registry))
                    )
                )
            )
        );

        return ();
    }
}