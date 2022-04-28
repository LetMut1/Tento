use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
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
use log4rs::config::Config as LogConfig;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use redis::ConnectionInfo;
use std::clone::Clone;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use tokio_postgres::Config as PostgresqlConfig;
use tokio_postgres::NoTls;
use tokio::signal;

pub struct Base;

impl Base {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "production.env";  // TODO Посмотреть, какие есть еще лучшие форматы аналоги .env (Может, Томл?)
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.env";
    const DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME: &'static str = "development.local.env";

    pub fn handle(
        binary_file_path: String
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = Self::load_and_check_environment_variables(binary_file_path.as_str()) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = Self::configure_log() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = Self::run_http_server() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    fn load_and_check_environment_variables<'a>(
        binary_file_path: &'a str
    ) -> Result<(), ErrorAuditor> {
        match Path::new(binary_file_path).parent() {
            Some(file_path) => {
                let production_environment_file_path_buffer = file_path.join(&Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME));
                if production_environment_file_path_buffer.exists() {
                    if let Err(error) = dotenv::from_path(production_environment_file_path_buffer.as_path()) {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }

                    env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_TRUE);
                } else {
                    let development_local_environment_file_path_buffer = file_path.join(&Path::new(Self::DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME));
                    if development_local_environment_file_path_buffer.exists() {
                        if let Err(error) = dotenv::from_path(development_local_environment_file_path_buffer.as_path()) {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    } else {
                        let development_environment_file_path_buffer = file_path.join(&Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME));
                        if development_environment_file_path_buffer.exists() {
                            if let Err(error) = dotenv::from_path(development_environment_file_path_buffer.as_path()) {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        } else {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Any ....env files does not exist.")},
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }

                    env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_FALSE);
                }

                if let Err(mut error) = Self::check_environment_variables() {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }

                return Ok(());
            }
            None => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::LogicError {logic_error: LogicError::new(false, "The directory does not exist.")},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
                
            }
        }
    }

    fn check_environment_variables(
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = EnvironmentVariableResolver::is_production() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_server_socket_address() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_logger_roller_log_file_name() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_logger_log_file_name() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_logger_encoder_pattern() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_security_jrwt_encoding_private_key() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_key() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_resource_postgresql_url() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_resource_redis_url() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
        if let Err(mut error) = EnvironmentVariableResolver::get_resource_email_server_socket_address() {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    fn configure_log(
    ) -> Result<(), ErrorAuditor> {
        match EnvironmentVariableResolver::get_logger_roller_log_file_name() {
            Ok(logger_roller_log_file_name) => {
                match FixedWindowRoller::builder()
                    .base(1)
                    .build(logger_roller_log_file_name.as_str(), 10) {          // TODO 10 - КОНСТАНТА или енваронмент
                    Ok(fixed_window_roller) => {
                        match EnvironmentVariableResolver::get_logger_encoder_pattern() {
                            Ok(logger_encoder_pattern) => {
                                match EnvironmentVariableResolver::get_logger_log_file_name() {
                                    Ok(logger_log_file_name) => {
                                        match RollingFileAppender::builder()
                                            .append(true)
                                            .encoder(Box::new(PatternEncoder::new(logger_encoder_pattern.as_str())))
                                            .build(
                                                logger_log_file_name,                                                                                           // TODO Через Тип неопходимый, а не СТринг
                                                Box::new(CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024 * 1024)), Box::new(fixed_window_roller)))
                                            ) {
                                            Ok(rolling_file_appender) => {
                                                let rolling_file_appender_name = "rfa";             // TODO  TODO  Const или Енваронмент
                                
                                                let appender = Appender::builder()
                                                    .build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));
                                        
                                                let root = Root::builder().appender(rolling_file_appender_name.to_string()).build(LevelFilter::Trace);  // TODO TODO TODO FIlter
                                        
                                                match LogConfig::builder()
                                                    .appender(appender)
                                                    .build(root) {
                                                    Ok(config) => {             
                                                        if let Err(error) = log4rs::init_config(config) {
                                                            return Err(
                                                                ErrorAuditor::new(
                                                                    ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                                                    BacktracePart::new(line!(), file!(), None)
                                                                )
                                                            );
                                                        }
                                                
                                                        return Ok(());
                                                    }
                                                    Err(error) => {
                                                        return Err(
                                                            ErrorAuditor::new(
                                                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                                                BacktracePart::new(line!(), file!(), None)
                                                            )
                                                        );
                                                    }
                                                }
                                            }
                                            Err(error) => {
                                                return Err(
                                                    ErrorAuditor::new(
                                                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                                        BacktracePart::new(line!(), file!(), None)
                                                    )
                                                );
                                            }
                                        }
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                        return Err(error);
                                    }
                                }
                            }
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                
                                return Err(error);
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error.root_cause())}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

     // TODO  TODO  TODO ---- create HTTP2 (h2).   // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
    #[tokio::main]                      // TODO написать без макроса
    async fn run_http_server(
    ) -> Result<(), ErrorAuditor> {
        match EnvironmentVariableResolver::get_resource_postgresql_url() {
            Ok(resource_postgresql_url) => {
                match PostgresqlConfig::from_str(resource_postgresql_url.as_str()) {
                    Ok(config) => {
                        match Pool::builder()    // TODO Для девелопмента ТЛС не нужен (НО можно подключить, как вариант), для Продакша - обязательно. Здесь Пул, который содержит только для Дев. Можно Пулы выделить в Оптион для дев и прод окруженияю. Либо через Дженерик, создавать и отдавать в зависимости от от ИзПродакшн значения. Либо Base it on a feature? Probably having NoTls be the feature, since it makes more sense to have TLS by default
                            .build(PostgresqlConnectionManager::new(config, NoTls)).await {                                          // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                            Ok(postgresql_connection_pool) => {
                                match EnvironmentVariableResolver::get_resource_redis_url() {
                                    Ok(resource_redis_url) => {
                                        match ConnectionInfo::from_str(resource_redis_url.as_str()) {
                                            Ok(connection_info) => {
                                                match RedisConnectionManager::new(connection_info) {
                                                    Ok(redis_connection_manager) => {
                                                        match Pool::builder()      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                                                            .build(redis_connection_manager).await {
                                                            Ok(redis_connection_pool) => {
                                                                match EnvironmentVariableResolver::get_server_socket_address() {
                                                                    Ok(server_socket_address) => {
                                                                        match SocketAddr::from_str(server_socket_address.as_str()) {
                                                                            Ok(socket_addres) => {
                        
                        
                                                                                // TODO  TODO  TODO ---------  убрать Замыкания, написав и стипизировав функцию (https://docs.rs/futures/latest/futures/future/type.BoxFuture.html может помочь). Либо так https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs Но здесь сущает future::Ready<>.
                                                                                let service = make_service_fn(move |_: &AddrStream| {
                                                                                    let postgresql_connection_pool = postgresql_connection_pool.clone();
                                                                        
                                                                                    let redis_connection_pool = redis_connection_pool.clone();
                                                                        
                                                                                    async move {
                                                                                        return Ok::<_, HyperError>(
                                                                                            service_fn(move |requset| {
                                                                                                let postgresql_connection_pool = postgresql_connection_pool.clone();
                                                                            
                                                                                                let redis_connection_pool = redis_connection_pool.clone();
                                                                            
                                                                                                return async move {
                                                                                                    return Ok::<_, HyperError>(Self::resolve(requset, postgresql_connection_pool, redis_connection_pool).await);
                                                                                                };
                                                                                            })
                                                                                        );
                                                                                    }
                                                                                });
                                                                                // TODO  TODO  TODO ------------------------------------------------------------------------------------------------------------------
                                                                        
                                                                        
                                                                        
                                                                        
                                                                                if let Err(error) = Server::bind(&socket_addres)       // TODO TODO TODO TODO TODO Настроить сервер для продакшна
                                                                                    .serve(service)
                                                                                    .with_graceful_shutdown(Self::create_shutdown_signal())
                                                                                    .await {
                                                                                        return Err(
                                                                                            ErrorAuditor::new(
                                                                                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                                                                                BacktracePart::new(line!(), file!(), None)
                                                                                            )
                                                                                        );
                                                                                    }
                                                                        
                                                                                return Ok(());
                                                                            }
                                                                            Err(error) => {
                                                                                return Err(
                                                                                    ErrorAuditor::new(
                                                                                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                                                                                        BacktracePart::new(line!(), file!(), None)
                                                                                    )
                                                                                );
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                        
                                                                        return Err(error);
                                                                    }
                                                                }
                                                            }
                                                            Err(error) => {
                                                                return Err(
                                                                    ErrorAuditor::new(
                                                                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error: error}}},
                                                                        BacktracePart::new(line!(), file!(), None)
                                                                    )
                                                                );
                                                            }
                                                        }
                                                    }
                                                    Err(error) => {
                                                        return Err(
                                                            ErrorAuditor::new(
                                                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error: error}}},
                                                                BacktracePart::new(line!(), file!(), None)
                                                            )
                                                        );
                                                    }
                                                }
                                            }
                                            Err(error) => {
                                                return Err(
                                                    ErrorAuditor::new(
                                                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error: error}}},
                                                        BacktracePart::new(line!(), file!(), None)
                                                    )
                                                );
                                            }
                                        }
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                        return Err(error);
                                    }
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error}}},
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error}}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    // TODO  TODO  TODO  УБрать expect. Перерегистрировать с помощью ТОКИО (без использования .with_graceful_shutdown()) ----------
    async fn create_shutdown_signal(
    ) -> () {
        signal::ctrl_c()
            .await
            .expect("Failed to install gracefully shutdown signal");

        return ();
    }

    async fn resolve(                                                           // TODO Пути через константы?
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>, 
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        match (request.uri().path(), request.method()) {
            // Area for existing routes with not authorized user.
            // GET functional. This is because there is a restriction on mobile frontend.
            ("/v1/m/au/cnfe", &Method::POST) => {
                return ControllerApplicationUserAuthorization::check_nickname_for_existing(request, postgresql_connection_pool).await;
            }
            // GET functional. This is because there is a restriction on mobile frontend.
            ("/v1/m/au/cefe", &Method::POST) => {
                return ControllerApplicationUserAuthorization::check_email_for_existing(request, postgresql_connection_pool).await;
            }
            ("/v1/m/au/rbfs", &Method::POST) => {
                return ControllerApplicationUserAuthorization::register_by_first_step(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rbls", &Method::POST) => {
                return ControllerApplicationUserAuthorization::register_by_last_step(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/sefr", &Method::POST) => {
                return ControllerApplicationUserAuthorization::send_email_for_register(request, redis_connection_pool).await;
            }
            ("/v1/m/au/libfs", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_in_by_first_step(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/libls", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_in_by_last_step(request, redis_connection_pool).await;
            }
            ("/v1/m/au/sefli", &Method::POST) => {
                return ControllerApplicationUserAuthorization::send_email_for_log_in(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rpbfs", &Method::POST) => {
                return ControllerApplicationUserAuthorization::reset_password_by_first_step(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rpbls", &Method::POST) => {
                return ControllerApplicationUserAuthorization::reset_password_by_last_step(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/sefrp", &Method::POST) => {
                return ControllerApplicationUserAuthorization::send_email_for_reset_password(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            ("/v1/m/au/rjawt", &Method::POST) => {
                return ControllerApplicationUserAuthorization::refresh_json_access_web_token(request, redis_connection_pool).await;
            }
            // Area for existing routes with authorized user.
            ("/v1/m/au/lofod", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_out_from_one_device(request, redis_connection_pool).await;
            }
            ("/v1/m/au/lofad", &Method::POST) => {
                return ControllerApplicationUserAuthorization::log_out_from_all_devices(request, redis_connection_pool).await;
            }
            // GET functional. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbn", &Method::POST) => {
                return ControllerChannelBase::get_many_by_name(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // GET functional. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbca", &Method::POST) => {
                return ControllerChannelBase::get_many_by_created_at(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // GET functional. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbsq", &Method::POST) => {
                return ControllerChannelBase::get_many_by_subscribers_quantity(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // GET functional. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbir", &Method::POST) => {
                return ControllerChannelBase::get_many_by_id_registry(request, postgresql_connection_pool, redis_connection_pool).await;
            }
            // Area for not existing routes.
            _ => {
                #[cfg(feature="facilitate_non_automatic_functional_testing")]
                match (request.uri().path(), request.method()) {
                    // Area for existing routes with not authorized user.
                    // GET functional. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cnfe_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::check_nickname_for_existing_(request, postgresql_connection_pool).await;
                    }
                    // GET functional. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cefe_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::check_email_for_existing_(request, postgresql_connection_pool).await;
                    }
                    ("/v1/m/au/rbfs_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::register_by_first_step_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rbls_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::register_by_last_step_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/sefr_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::send_email_for_register_(request, redis_connection_pool).await;
                    }
                    ("/v1/m/au/libfs_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_in_by_first_step_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/libls_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_in_by_last_step_(request, redis_connection_pool).await;
                    }
                    ("/v1/m/au/sefli_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::send_email_for_log_in_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rpbfs_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::reset_password_by_first_step_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rpbls_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::reset_password_by_last_step_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/sefrp_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::send_email_for_reset_password_(request, postgresql_connection_pool, redis_connection_pool).await;
                    }
                    ("/v1/m/au/rjawt_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::refresh_json_access_web_token_(request, redis_connection_pool).await;
                    }
                    // Area for existing routes with authorized user.
                    ("/v1/m/au/lofod_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_out_from_one_device_(request, redis_connection_pool).await;
                    }
                    ("/v1/m/au/lofad_", &Method::POST) => {
                        return ControllerApplicationUserAuthorization::log_out_from_all_devices_(request, redis_connection_pool).await;
                    }
                    // Area for not existing routes.
                    _ => {}
                }

                return ControllerRouteNotFound::not_found().await;
            }
        }
    }
}