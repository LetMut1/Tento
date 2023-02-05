use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::functionality::action::mobile::version_1::application_user__authorization;
use crate::presentation_layer::functionality::action::mobile::version_1::channel__base;
use crate::presentation_layer::functionality::action::route_not_found::route_not_found;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::dotenv;
use extern_crate::hyper::Body;
use extern_crate::hyper::Error as HyperError;
use extern_crate::hyper::Method;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::hyper::Server;
use extern_crate::hyper::server::conn::AddrStream;
use extern_crate::hyper::service::make_service_fn;
use extern_crate::hyper::service::service_fn;
use extern_crate::log::LevelFilter;
use extern_crate::log4rs;
use extern_crate::log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use extern_crate::log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use extern_crate::log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use extern_crate::log4rs::append::rolling_file::RollingFileAppender;
use extern_crate::log4rs::config::Appender;
use extern_crate::log4rs::config::Config as LogConfiguration;
use extern_crate::log4rs::config::Root;
use extern_crate::log4rs::encode::pattern::PatternEncoder;
use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::Config as PostgresqlConfiguration;
use extern_crate::tokio_postgres::NoTls;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio;
use extern_crate::tokio::signal;
use std::clone::Clone;
use std::env;
use std::marker::Send;
use std::marker::Sync;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::str::FromStr;

pub struct RunServerProcessor;

impl RunServerProcessor {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "production.env";  // TODO Посмотреть, какие есть еще лучшие форматы аналоги .env (Может, Томл?)
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.env";
    const LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.local.env";

    pub fn process(binary_file_path: &'static str) -> Result<(), ErrorAuditor> {
        let environment_configuration_resolver = match Self::load_environment_configuration_registry(binary_file_path) {
            Ok(environment_configuration_resolver_) => environment_configuration_resolver_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

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

    fn load_environment_configuration_registry<'a>(binary_file_path: &'a str) -> Result<EnvironmentConfigurationResolver, ErrorAuditor> {
        let file_path = match Path::new(binary_file_path).parent() {
            Some(file_path_) => file_path_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "The directory does not exist.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let production_environment_file_path_buffer = file_path.join(
            Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME)
        );

        let is_production_environment = if production_environment_file_path_buffer.exists() {
            if let Err(error) = dotenv::from_path(production_environment_file_path_buffer.as_path()) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            true
        } else {
            let development_local_environment_file_path_buffer = file_path.join(
                Path::new(Self::LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME)
            );

            if development_local_environment_file_path_buffer.exists() {
                if let Err(error) = dotenv::from_path(development_local_environment_file_path_buffer.as_path()) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            } else {
                let development_environment_file_path_buffer = file_path.join(
                    Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME)
                );

                if development_environment_file_path_buffer.exists() {
                    if let Err(error) = dotenv::from_path(development_environment_file_path_buffer.as_path()) {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                } else {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError { logic_error: LogicError::new(false, "Any ....env files does not exist.") },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }

            false
        };

        let application_server_socket_address = match env::var(EnvironmentConfigurationResolver::APPLICATION_SERVER_SOCKET_ADDRESS_KEY) {
            Ok(application_server_socket_address_) => application_server_socket_address_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let mut application_server_socket_address_registry = match application_server_socket_address.to_socket_addrs() {
            Ok(application_server_socket_address_registry_) => application_server_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_server_socket_address_ = match application_server_socket_address_registry.next() {
            Some(application_server_socket_address__) => application_server_socket_address__,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "Invalid socket address.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::APPLICATION_SERVER_SOCKET_ADDRESS_KEY);            // TODO TODO TODO TODO TODOenv::remove_var can PANIC. Подумать, что делать. Использовать другой крейт (toml), или написать свой парсер. Паника - всегжа плохо

        let logger_roller_log_file_name = match env::var(EnvironmentConfigurationResolver::LOGGER_ROLLER_LOG_FILE_NAME_KEY) {
            Ok(logger_roller_log_file_name_) => logger_roller_log_file_name_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::LOGGER_ROLLER_LOG_FILE_NAME_KEY);

        let logger_log_file_name = match env::var(EnvironmentConfigurationResolver::LOGGER_LOG_FILE_NAME_KEY) {
            Ok(logger_log_file_name_) => logger_log_file_name_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::LOGGER_LOG_FILE_NAME_KEY);

        let logger_encoder_pattern = match env::var(EnvironmentConfigurationResolver::LOGGER_ENCODER_PATTERN_KEY) {
            Ok(logger_encoder_pattern_) => logger_encoder_pattern_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::LOGGER_ENCODER_PATTERN_KEY);

        let security_auart_encoding_private_key = match env::var(EnvironmentConfigurationResolver::SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY) {
            Ok(security_auart_encoding_private_key_) => security_auart_encoding_private_key_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY);

        let security_auat_signature_encoding_private_key = match env::var(EnvironmentConfigurationResolver::SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY) {
            Ok(security_auat_signature_encoding_private_key_) => security_auat_signature_encoding_private_key_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY);

        let resource_database_1_postgresql_url = match env::var(EnvironmentConfigurationResolver::RESOURCE_DATABASE_1_POSTGRESQL_URL_KEY) {
            Ok(resource_database_1_postgresql_url_) => resource_database_1_postgresql_url_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_database_1_postgresql_configuration = match PostgresqlConfiguration::from_str(resource_database_1_postgresql_url.as_str()) {
            Ok(postgresql_configuration) => postgresql_configuration,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::RESOURCE_DATABASE_1_POSTGRESQL_URL_KEY);

        let resource_database_2_postgresql_url = match env::var(EnvironmentConfigurationResolver::RESOURCE_DATABASE_2_POSTGRESQL_URL_KEY) {
            Ok(resource_database_2_postgresql_url_) =>  resource_database_2_postgresql_url_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_database_2_postgresql_configuration = match PostgresqlConfiguration::from_str(resource_database_2_postgresql_url.as_str()) {
            Ok(postgresql_configuration) => postgresql_configuration,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::RESOURCE_DATABASE_2_POSTGRESQL_URL_KEY);

        let resource_redis_url = match env::var(EnvironmentConfigurationResolver::RESOURCE_REDIS_URL_KEY) {
            Ok(resource_redis_url_) => resource_redis_url_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_redis_connection_info = match ConnectionInfo::from_str(resource_redis_url.as_str()) {
            Ok(connection_info) => connection_info,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::RESOURCE_REDIS_URL_KEY);

        let resource_email_server_socket_address = match env::var(EnvironmentConfigurationResolver::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY) {
            Ok(resource_email_server_socket_address_) => resource_email_server_socket_address_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let mut resource_email_server_socket_address_registry = match resource_email_server_socket_address.to_socket_addrs() {
            Ok(resource_email_server_socket_address_registry_) => resource_email_server_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_email_server_socket_address_ = match resource_email_server_socket_address_registry.next() {
            Some(resource_email_server_socket_address__) => resource_email_server_socket_address__,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "Invalid socket address.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfigurationResolver::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY);

        return Ok(
            EnvironmentConfigurationResolver::new(
                is_production_environment,
                application_server_socket_address_,
                logger_roller_log_file_name,
                logger_log_file_name,
                logger_encoder_pattern,
                security_auart_encoding_private_key,
                security_auat_signature_encoding_private_key,
                resource_database_1_postgresql_configuration,
                resource_database_2_postgresql_configuration,
                resource_redis_connection_info,
                resource_email_server_socket_address_
            )
        );
    }

    fn configure_log<'a>(environment_configuration_resolver: &'a EnvironmentConfigurationResolver) -> Result<(), ErrorAuditor> {
        let fixed_window_roller = match FixedWindowRoller::builder()
            .base(1)
            .build(environment_configuration_resolver.get_logger_roller_log_file_name(), 10) {          // TODO 10 - КОНСТАНТА или енваронмент
            Ok(fixed_window_roller_) => fixed_window_roller_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error.root_cause()) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let rolling_file_appender = match RollingFileAppender::builder()
            .append(true)
            .encoder(
                Box::new(
                    PatternEncoder::new(environment_configuration_resolver.get_logger_encoder_pattern())
                )
            )
            .build(
                Path::new(environment_configuration_resolver.get_logger_log_file_name()),                                                                                      // TODO Через Тип неопходимый, а не СТринг
                Box::new(
                    CompoundPolicy::new(
                        Box::new(
                            SizeTrigger::new(50 * 1024 * 1024)
                        ),
                        Box::new(fixed_window_roller)
                    )
                )
            ) {
            Ok(rolling_file_appender_) => rolling_file_appender_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let rolling_file_appender_name = "rfa";            // TODO  TODO  Const или Енваронмент

        let appender = Appender::builder()
            .build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));

        let root = Root::builder()
            .appender(rolling_file_appender_name.to_string())
            .build(LevelFilter::Error);                                 // TODO TODO TODO FIlter

        let log_configuration = match LogConfiguration::builder()
            .appender(appender)
            .build(root) {
            Ok(log_configuration_) => log_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = log4rs::init_config(log_configuration) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }

    // TODO  TODO  TODO ---- create HTTP2 (h2).   // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
    // TODO написать без макроса
    #[tokio::main]
    async fn run_http_server(environment_configuration_resolver: EnvironmentConfigurationResolver) -> Result<(), ErrorAuditor> {
        let postgresql_connection_pool_workflow_type_aggregator = if environment_configuration_resolver.is_production_environment() {
            todo!();           // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
        } else {
            let database_1_postgresql_connection_pool = match Pool::builder()
                .build(
                    PostgresqlConnectionManager::new(
                        environment_configuration_resolver.get_resource_database_1_postgresql_configuration().clone(), NoTls
                    )
                ).await {         // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                Ok(database_1_postgresql_connection_pool_) => database_1_postgresql_connection_pool_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let database_2_postgresql_connection_pool = match Pool::builder()
                .build(
                    PostgresqlConnectionManager::new(
                        environment_configuration_resolver.get_resource_database_2_postgresql_configuration().clone(), NoTls
                    )
                ).await {
                Ok(database_2_postgresql_connection_pool_) => database_2_postgresql_connection_pool_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            PostgresqlConnectionPoolWorkflowTypeAggregator::LocalDevelopment { database_1_postgresql_connection_pool, database_2_postgresql_connection_pool }
        };

        let redis_connection_manager = match RedisConnectionManager::new(environment_configuration_resolver.get_resource_redis_url().clone()) {
            Ok(redis_connection_manager_) => redis_connection_manager_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let redis_connection_pool = match Pool::builder()      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
            .build(redis_connection_manager).await {
            Ok(redis_connection_pool_) => redis_connection_pool_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let builder = Server::bind(environment_configuration_resolver.get_application_server_socket_address());

        // TODO  TODO  TODO ---------  убрать Замыкания, написав и стипизировав функцию (https://docs.rs/futures/latest/futures/future/type.BoxFuture.html может помочь). Либо так https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs Но здесь сущает future::Ready<>.
        let service = make_service_fn(
            move |_: &AddrStream| {
                let environment_configuration_resolver_ = environment_configuration_resolver.clone();

                let postgresql_connection_pool_workflow_type_aggregator_ = postgresql_connection_pool_workflow_type_aggregator.clone();

                let redis_connection_pool_ = redis_connection_pool.clone();

                async move {
                    return Ok::<_, HyperError>(
                        service_fn(
                            move |requset| {
                                let environment_configuration_resolver__ = environment_configuration_resolver_.clone();

                                let postgresql_connection_pool_workflow_type_aggregator__ = postgresql_connection_pool_workflow_type_aggregator_.clone();

                                let redis_connection_pool__ = redis_connection_pool_.clone();

                                return async move {
                                    let (database_1_postgresql_connection_pool_, database_2_postgresql_connection_pool_) = match postgresql_connection_pool_workflow_type_aggregator__ {
                                        PostgresqlConnectionPoolWorkflowTypeAggregator::LocalDevelopment {
                                            database_1_postgresql_connection_pool, database_2_postgresql_connection_pool
                                        } => (database_1_postgresql_connection_pool, database_2_postgresql_connection_pool)
                                    };

                                    return Ok::<_, HyperError>(
                                        Self::resolve(
                                            &environment_configuration_resolver__,
                                            requset,
                                            database_1_postgresql_connection_pool_,
                                            database_2_postgresql_connection_pool_,
                                            redis_connection_pool__
                                        ).await
                                    );
                                };
                            }
                        )
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
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

        return Ok(());
    }

    async fn create_shutdown_signal() -> () {    // TODO  TODO  TODO  УБрать expect. Перерегистрировать с помощью ТОКИО (без использования .with_graceful_shutdown()) ----------
        signal::ctrl_c()
            .await
            .expect("Failed to install gracefully shutdown signal");

        return ();
    }

    async fn resolve<'a, T>(   // TODO Можно ли пробростить ЛОггер как объект? Нужно ли?  (Лог4рс делает так, чтобы все крееты, на основе этого лога могли писать в общий лог) // TODO TODO  TODO Пути через константы?
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        database_1_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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
                return application_user__authorization::check_nickname_for_existing::check_nickname_for_existing(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/au/cefe", &Method::POST) => {
                return application_user__authorization::check_email_for_existing::check_email_for_existing(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rbfs", &Method::POST) => {
                return application_user__authorization::register_by_first_step::register_by_first_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rbss", &Method::POST) => {
                return application_user__authorization::register_by_second_step::register_by_second_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rbls", &Method::POST) => {
                return application_user__authorization::register_by_last_step::register_by_last_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/sefr", &Method::POST) => {
                return application_user__authorization::send_email_for_register::send_email_for_register(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/abfs", &Method::POST) => {
                return application_user__authorization::authorize_by_first_step::authorize_by_first_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/abls", &Method::POST) => {
                return application_user__authorization::authorize_by_last_step::authorize_by_last_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/sefa", &Method::POST) => {
                return application_user__authorization::send_email_for_authorize::send_email_for_authorize(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rpbfs", &Method::POST) => {
                return application_user__authorization::reset_password_by_first_step::reset_password_by_first_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rpbss", &Method::POST) => {
                return application_user__authorization::reset_password_by_second_step::reset_password_by_second_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rpbls", &Method::POST) => {
                return application_user__authorization::reset_password_by_last_step::reset_password_by_last_step(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/sefrp", &Method::POST) => {
                return application_user__authorization::send_email_for_reset_password::send_email_for_reset_password(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rauat", &Method::POST) => {
                return application_user__authorization::refresh_application_user_access_token::refresh_application_user_access_token(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // Area for existing routes with authorized user.
            ("/v1/m/au/dfod", &Method::POST) => {
                return application_user__authorization::deauthorize_from_one_device::deauthorize_from_one_device(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/dfad", &Method::POST) => {
                return application_user__authorization::deauthorize_from_all_devices::deauthorize_from_all_devices(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbn", &Method::POST) => {
                return channel__base::get_many_by_name::get_many_by_name(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbca", &Method::POST) => {
                return channel__base::get_many_by_created_at::get_many_by_created_at(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbsq", &Method::POST) => {
                return channel__base::get_many_by_subscribers_quantity::get_many_by_subscribers_quantity(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbir", &Method::POST) => {
                return channel__base::get_many_by_id_registry::get_many_by_id_registry(
                    environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // Area for not existing routes.
            _ => {
                #[cfg(feature = "facilitate_non_automatic_functional_testing")]
                match (request.uri().path(), request.method()) {
                    // Area for existing routes with not authorized user.
                    // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cnfe_", &Method::POST) => {
                        return application_user__authorization::check_nickname_for_existing::check_nickname_for_existing_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cefe_", &Method::POST) => {
                        return application_user__authorization::check_email_for_existing::check_email_for_existing_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rbfs_", &Method::POST) => {
                        return application_user__authorization::register_by_first_step::register_by_first_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rbss_", &Method::POST) => {
                        return application_user__authorization::register_by_second_step::register_by_second_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rbls_", &Method::POST) => {
                        return application_user__authorization::register_by_last_step::register_by_last_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/sefr_", &Method::POST) => {
                        return application_user__authorization::send_email_for_register::send_email_for_register_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/abfs_", &Method::POST) => {
                        return application_user__authorization::authorize_by_first_step::authorize_by_first_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/abls_", &Method::POST) => {
                        return application_user__authorization::authorize_by_last_step::authorize_by_last_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/sefa_", &Method::POST) => {
                        return application_user__authorization::send_email_for_authorize::send_email_for_authorize_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rpbfs_", &Method::POST) => {
                        return application_user__authorization::reset_password_by_first_step::reset_password_by_first_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rpbss_", &Method::POST) => {
                        return application_user__authorization::reset_password_by_second_step::reset_password_by_second_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rpbls_", &Method::POST) => {
                        return application_user__authorization::reset_password_by_last_step::reset_password_by_last_step_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/sefrp_", &Method::POST) => {
                        return application_user__authorization::send_email_for_reset_password::send_email_for_reset_password_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rauat_", &Method::POST) => {
                        return application_user__authorization::refresh_application_user_access_token::refresh_application_user_access_token_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // Area for existing routes with authorized user.
                    ("/v1/m/au/dfod_", &Method::POST) => {
                        return application_user__authorization::deauthorize_from_one_device::deauthorize_from_one_device_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/dfad_", &Method::POST) => {
                        return application_user__authorization::deauthorize_from_all_devices::deauthorize_from_all_devices_(
                            environment_configuration_resolver, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // Area for not existing routes.
                    _ => {}
                }

                return route_not_found().await;
            }
        }
    }
}

#[derive(Clone)]
enum PostgresqlConnectionPoolWorkflowTypeAggregator {
    LocalDevelopment {
        database_1_postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        database_2_postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    }
}