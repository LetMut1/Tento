use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::_code_optimization::PostgresqlConnectionPool;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization as ControllerApplicationUserAuthorization;
use crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::Base as ControllerChannelBase;
use crate::presentation_layer::service::controller::route_not_found::RouteNotFound as ControllerRouteNotFound;
use hyper::Body;
use hyper::Error as HyperError;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::Appender;
use log4rs::config::Config as LogConfiguration;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use redis::ConnectionInfo;
use std::clone::Clone;
use std::env;
use std::marker::Send;
use std::marker::Sync;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::str::FromStr;
use tokio_postgres::Config as PostgresqlConfiguration;
use tokio_postgres::NoTls;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio::signal;

pub struct Base;

impl Base {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "production.env";  // TODO Посмотреть, какие есть еще лучшие форматы аналоги .env (Может, Томл?)
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.env";
    const DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME: &'static str = "development.local.env";

    pub fn handle(
        binary_file_path: &'static str
    ) -> Result<(), ErrorAuditor> {
        match Self::load_environment_configuration_registry(binary_file_path) {
            Ok(environment_configuration_resolver) => {
                if let Err(mut error) = Self::configure_log(&environment_configuration_resolver) {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                    return Err(error);
                }
                if let Err(mut error) = Self::run_http_server(environment_configuration_resolver) {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                    return Err(error);
                }
        
                return Ok(());
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    fn load_environment_configuration_registry<'a>(
        binary_file_path: &'a str
    ) -> Result<EnvironmentConfigurationResolver, ErrorAuditor> {
        match Path::new(binary_file_path).parent() {
            Some(file_path) => {
                let is_production_environment: bool;

                let production_environment_file_path_buffer = file_path.join(Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME));
                if production_environment_file_path_buffer.exists() {
                    if let Err(error) = dotenv::from_path(production_environment_file_path_buffer.as_path()) {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }

                    is_production_environment = true;
                } else {
                    let development_local_environment_file_path_buffer = file_path.join(Path::new(Self::DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME));
                    if development_local_environment_file_path_buffer.exists() {
                        if let Err(error) = dotenv::from_path(development_local_environment_file_path_buffer.as_path()) {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    } else {
                        let development_environment_file_path_buffer = file_path.join(Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME));
                        if development_environment_file_path_buffer.exists() {
                            if let Err(error) = dotenv::from_path(development_environment_file_path_buffer.as_path()) {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        } else {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(false, "Any ....env files does not exist.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }

                    is_production_environment = false;
                }

                let application_server_socket_address: SocketAddr;
                match env::var(EnvironmentConfigurationResolver::APPLICATION_SERVER_SOCKET_ADDRESS_KEY) {
                    Ok(application_server_socket_address_) => {
                        match application_server_socket_address_.to_socket_addrs() {
                            Ok(mut application_server_socket_address_registry) => {
                                match application_server_socket_address_registry.next() {
                                    Some(application_server_socket_address___) => {
                                        application_server_socket_address = application_server_socket_address___;

                                        env::remove_var(EnvironmentConfigurationResolver::APPLICATION_SERVER_SOCKET_ADDRESS_KEY);            // TODO TODO TODO TODO TODOenv::remove_var can PANIC. Подумать, что делать. Использовать другой крейт (toml), или написать свой парсер. Паника - всегжа плохо
                                    }
                                    None => {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::LogicError { logic_error: LogicError::new(false, "Invalid socket address.") },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let logger_roller_log_file_name: String;
                match env::var(EnvironmentConfigurationResolver::LOGGER_ROLLER_LOG_FILE_NAME_KEY) {
                    Ok(logger_roller_log_file_name_) => {
                        logger_roller_log_file_name = logger_roller_log_file_name_;

                        env::remove_var(EnvironmentConfigurationResolver::LOGGER_ROLLER_LOG_FILE_NAME_KEY);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let logger_log_file_name: String;
                match env::var(EnvironmentConfigurationResolver::LOGGER_LOG_FILE_NAME_KEY) {
                    Ok(logger_log_file_name_) => {
                        logger_log_file_name = logger_log_file_name_;

                        env::remove_var(EnvironmentConfigurationResolver::LOGGER_LOG_FILE_NAME_KEY);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let logger_encoder_pattern: String;
                match env::var(EnvironmentConfigurationResolver::LOGGER_ENCODER_PATTERN_KEY) {
                    Ok(logger_encoder_pattern_) => {
                        logger_encoder_pattern = logger_encoder_pattern_;

                        env::remove_var(EnvironmentConfigurationResolver::LOGGER_ENCODER_PATTERN_KEY);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let security_jrwt_encoding_private_key: String;
                match env::var(EnvironmentConfigurationResolver::SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY) {
                    Ok(security_jrwt_encoding_private_key_) => {
                        security_jrwt_encoding_private_key = security_jrwt_encoding_private_key_;

                        env::remove_var(EnvironmentConfigurationResolver::SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let security_jawt_signature_encoding_private_key: String;
                match env::var(EnvironmentConfigurationResolver::SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY) {
                    Ok(security_jawt_signature_encoding_private_key_) => {
                        security_jawt_signature_encoding_private_key = security_jawt_signature_encoding_private_key_;

                        env::remove_var(EnvironmentConfigurationResolver::SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let resource_postgresql_configuration: PostgresqlConfiguration;
                match env::var(EnvironmentConfigurationResolver::RESOURCE_POSTGRESQL_URL_KEY) {
                    Ok(resource_postgresql_url) => {
                        match PostgresqlConfiguration::from_str(resource_postgresql_url.as_str()) {
                            Ok(postgresql_configuration) => {
                                resource_postgresql_configuration = postgresql_configuration;

                                env::remove_var(EnvironmentConfigurationResolver::RESOURCE_POSTGRESQL_URL_KEY);
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let resource_redis_url: String;
                match env::var(EnvironmentConfigurationResolver::RESOURCE_REDIS_URL_KEY) {
                    Ok(resource_redis_url_) => {
                        resource_redis_url = resource_redis_url_;

                        env::remove_var(EnvironmentConfigurationResolver::RESOURCE_REDIS_URL_KEY);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                let resource_email_server_socket_address: SocketAddr;
                match env::var(EnvironmentConfigurationResolver::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY) {
                    Ok(resource_email_server_socket_address_) => {
                        match resource_email_server_socket_address_.to_socket_addrs() {
                            Ok(mut resource_email_server_socket_address_registry) => {
                                match resource_email_server_socket_address_registry.next() {
                                    Some(resource_email_server_socket_address___) => {
                                        resource_email_server_socket_address = resource_email_server_socket_address___;

                                        env::remove_var(EnvironmentConfigurationResolver::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY);
                                    }
                                    None => {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::LogicError { logic_error: LogicError::new(false, "Invalid socket address.") },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }

                return Ok(
                    EnvironmentConfigurationResolver::new(
                        is_production_environment,
                        application_server_socket_address,
                        logger_roller_log_file_name,
                        logger_log_file_name,
                        logger_encoder_pattern,
                        security_jrwt_encoding_private_key,
                        security_jawt_signature_encoding_private_key,
                        resource_postgresql_configuration,
                        resource_redis_url,
                        resource_email_server_socket_address
                    )
                );
            }
            None => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::LogicError { logic_error: LogicError::new(false, "The directory does not exist.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    fn configure_log<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver
    ) -> Result<(), ErrorAuditor> {
        match FixedWindowRoller::builder()
            .base(1)
            .build(environment_configuration_resolver.get_logger_roller_log_file_name(), 10) {          // TODO 10 - КОНСТАНТА или енваронмент
            Ok(fixed_window_roller) => {
                match RollingFileAppender::builder()
                    .append(true)
                    .encoder(Box::new(PatternEncoder::new(environment_configuration_resolver.get_logger_encoder_pattern())))
                    .build(
                        Path::new(environment_configuration_resolver.get_logger_log_file_name()),                                                                                      // TODO Через Тип неопходимый, а не СТринг
                        Box::new(CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024 * 1024)), Box::new(fixed_window_roller)))
                    ) {
                    Ok(rolling_file_appender) => {
                        let rolling_file_appender_name = "rfa";            // TODO  TODO  Const или Енваронмент
        
                        let appender = Appender::builder()
                            .build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));
                
                        let root = Root::builder().appender(rolling_file_appender_name.to_string()).build(LevelFilter::Trace);  // TODO TODO TODO FIlter
                
                        match LogConfiguration::builder()
                            .appender(appender)
                            .build(root) {
                            Ok(config) => {             
                                if let Err(error) = log4rs::init_config(config) {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                        
                                return Ok(());
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error.root_cause()) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

     // TODO  TODO  TODO ---- create HTTP2 (h2).   // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
    #[tokio::main]                      // TODO написать без макроса
    async fn run_http_server(
        environment_configuration_resolver: EnvironmentConfigurationResolver
    ) -> Result<(), ErrorAuditor> {
        let postgresql_connection_pool: PostgresqlConnectionPool;
        if environment_configuration_resolver.is_production_environment() {
            todo!();           // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
        } else {
            match Pool::builder()
                .build(
                    PostgresqlConnectionManager::new(
                        environment_configuration_resolver.get_resource_postgresql_configuration().clone(), NoTls
                    )
                ).await {         // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                Ok(postgresql_connection_pool_) => {
                    postgresql_connection_pool = PostgresqlConnectionPool::Development { postgresql_connection_pool: postgresql_connection_pool_ };
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        match ConnectionInfo::from_str(environment_configuration_resolver.get_resource_redis_url()) {
            Ok(connection_info) => {
                match RedisConnectionManager::new(connection_info) {
                    Ok(redis_connection_manager) => {
                        match Pool::builder()      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                            .build(redis_connection_manager).await {
                            Ok(redis_connection_pool) => {
                                let builder = Server::bind(environment_configuration_resolver.get_application_server_socket_address());

                                // TODO  TODO  TODO ---------  убрать Замыкания, написав и стипизировав функцию (https://docs.rs/futures/latest/futures/future/type.BoxFuture.html может помочь). Либо так https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs Но здесь сущает future::Ready<>.
                                let service = make_service_fn(
                                    move |_: &AddrStream| {
                                        let environment_configuration_resolver_ = environment_configuration_resolver.clone();

                                        let postgresql_connection_pool_ = postgresql_connection_pool.clone();
                            
                                        let redis_connection_pool_ = redis_connection_pool.clone();
                            
                                        async move {
                                            return Ok::<_, HyperError>(
                                                service_fn(move |requset| {
                                                    let environment_configuration_resolver__ = environment_configuration_resolver_.clone();

                                                    let postgresql_connection_pool__ = postgresql_connection_pool_.clone();
                                
                                                    let redis_connection_pool__ = redis_connection_pool_.clone();
                                
                                                    return async move {
                                                        match postgresql_connection_pool__ {
                                                            PostgresqlConnectionPool::Development { postgresql_connection_pool } => {
                                                                return Ok::<_, HyperError>(Self::resolve(&environment_configuration_resolver__, requset, postgresql_connection_pool, redis_connection_pool__).await);
                                                            }
                                                        }
                                                    };
                                                })
                                            );
                                        }
                                    }
                                );
                                // TODO  TODO  TODO ------------------------------------------------------------------------------------------------------------------
                        
                        
                        
                                if let Err(error) = builder       // TODO TODO TODO TODO TODO Настроить сервер для продакшна
                                    .serve(service)
                                    .with_graceful_shutdown(Self::create_shutdown_signal())
                                    .await {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                        
                                return Ok(());
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    async fn create_shutdown_signal(    // TODO  TODO  TODO  УБрать expect. Перерегистрировать с помощью ТОКИО (без использования .with_graceful_shutdown()) ----------
    ) -> () {
        signal::ctrl_c()
            .await
            .expect("Failed to install gracefully shutdown signal");

        return ();
    }

    async fn resolve<'a, T>(                                                           // TODO TODO  TODO Пути через константы?
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>, 
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        match (request.uri().path(), request.method()) {
            // Area for existing routes with not authorized user.
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/au/cnfe", &Method::POST) => {
                return ControllerApplicationUserAuthorization::check_nickname_for_existing(environment_configuration_resolver, request, postgresql_connection_pool).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/au/cefe", &Method::POST) => {
                return ControllerApplicationUserAuthorization::check_email_for_existing(environment_configuration_resolver, request, postgresql_connection_pool).await;
            }
            ("/v1/m/au/rbfs", &Method::POST) => {
                return ControllerApplicationUserAuthorization::register_by_first_step(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rbls", &Method::POST) => {
                return ControllerApplicationUserAuthorization::register_by_last_step(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/sefr", &Method::POST) => {
                return ControllerApplicationUserAuthorization::send_email_for_register(environment_configuration_resolver, request, redis_connection_pool).await;
            }
            ("/v1/m/au/libfs", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_in_by_first_step(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/libls", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_in_by_last_step(environment_configuration_resolver, request, redis_connection_pool).await;
            }
            ("/v1/m/au/sefli", &Method::POST) => {
                return ControllerApplicationUserAuthorization::send_email_for_log_in(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rpbfs", &Method::POST) => {
                return ControllerApplicationUserAuthorization::reset_password_by_first_step(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rpbls", &Method::POST) => {
                return ControllerApplicationUserAuthorization::reset_password_by_last_step(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/sefrp", &Method::POST) => {
                return ControllerApplicationUserAuthorization::send_email_for_reset_password(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rjawt", &Method::POST) => {
                return ControllerApplicationUserAuthorization::refresh_json_access_web_token(environment_configuration_resolver, request, redis_connection_pool).await;
            }
            // Area for existing routes with authorized user.
            ("/v1/m/au/lofod", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_out_from_one_device(environment_configuration_resolver, request, redis_connection_pool).await;
            }
            ("/v1/m/au/lofad", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_out_from_all_devices(environment_configuration_resolver, request, redis_connection_pool).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbn", &Method::POST) => {
                return ControllerChannelBase::get_many_by_name(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbca", &Method::POST) => {
                return ControllerChannelBase::get_many_by_created_at(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbsq", &Method::POST) => {
                return ControllerChannelBase::get_many_by_subscribers_quantity(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbir", &Method::POST) => {
                return ControllerChannelBase::get_many_by_id_registry(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/abc_a", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abc_a().await;
            }
            ("/abc_b", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abc_b().await;
            }
            ("/abc_c", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abc_c().await;
            }
            ("/abc_inner_a", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abc_inner_a().await;
            }
            ("/abc_inner_b", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abc_inner_b().await;
            }
            ("/abc_inner_c", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abc_inner_c().await;
            }
            ("/abcdefgh_1", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abcdefgh_1().await;
            }
            ("/abcdefgh_2", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abcdefgh_2().await;
            }
            ("/abcdefgh_3", &Method::POST) => {
                return crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::
                _new_for_context::authorization::message_pack_testing_purpose::abcdefgh_3().await;
            }
            // Area for not existing routes.
            _ => {
                #[cfg(feature="facilitate_non_automatic_functional_testing")]
                match (request.uri().path(), request.method()) {
                    // Area for existing routes with not authorized user.
                    // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cnfe_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::check_nickname_for_existing_(environment_configuration_resolver, request, postgresql_connection_pool).await;
                    }
                    // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cefe_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::check_email_for_existing_(environment_configuration_resolver, request, postgresql_connection_pool).await;
                    }
                    ("/v1/m/au/rbfs_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::register_by_first_step_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rbls_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::register_by_last_step_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/sefr_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::send_email_for_register_(environment_configuration_resolver, request, redis_connection_pool).await;
                    }
                    ("/v1/m/au/libfs_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_in_by_first_step_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/libls_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_in_by_last_step_(environment_configuration_resolver, request, redis_connection_pool).await;
                    }
                    ("/v1/m/au/sefli_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::send_email_for_log_in_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rpbfs_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::reset_password_by_first_step_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rpbls_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::reset_password_by_last_step_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/sefrp_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::send_email_for_reset_password_(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rjawt_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::refresh_json_access_web_token_(environment_configuration_resolver, request, redis_connection_pool).await;
                    }
                    // Area for existing routes with authorized user.
                    ("/v1/m/au/lofod_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_out_from_one_device_(environment_configuration_resolver, request, redis_connection_pool).await;
                    }
                    ("/v1/m/au/lofad_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_out_from_all_devices_(environment_configuration_resolver, request, redis_connection_pool).await;
                    }
                    // Area for not existing routes.
                    _ => {}
                }

                return ControllerRouteNotFound::not_found().await;
            }
        }
    }
}