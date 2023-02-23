use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::environment_configuration__creator::EnvironmentConfiguration_Creator;
use crate::presentation_layer::functionality::action::mobile::version_1::application_user__authorization;
use crate::presentation_layer::functionality::action::mobile::version_1::channel__base;
use crate::presentation_layer::functionality::action::route_not_found;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::hyper::Body;
use extern_crate::hyper::Error as HyperError;
use extern_crate::hyper::Method;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::hyper::Server;
use extern_crate::hyper::server::conn::AddrStream;
use extern_crate::hyper::service::make_service_fn;
use extern_crate::hyper::service::service_fn;
use extern_crate::tokio_postgres::NoTls;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio::runtime::Builder;
use extern_crate::tokio::signal;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct RunServerProcessor;

impl RunServerProcessor {
    pub fn process(binary_file_path: &'static str) -> Result<(), ErrorAuditor> {
        let environment_configuration = match EnvironmentConfiguration_Creator::create_from_configuration_file(binary_file_path) {
            Ok(environment_configuration_) => environment_configuration_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let runtime = match Builder::new_multi_thread()
            .enable_all()
            .build() {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(mut error) = runtime.block_on(
            Self::run_http_server(&environment_configuration)
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    async fn run_http_server<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<(), ErrorAuditor> {     // TODO  TODO  TODO ---- create HTTP2 (h2).   // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
        let postgresql_connection_pool_workflow_type_aggregator = if environment_configuration.is_production_environment() {
            todo!();           // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
        } else {
            let database_1_postgresql_connection_pool = match Pool::builder()
                .build(
                    PostgresqlConnectionManager::new(
                        environment_configuration.get_resource_database_1_postgresql_configuration().clone(), NoTls
                    )
                ).await {         // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                Ok(database_1_postgresql_connection_pool_) => database_1_postgresql_connection_pool_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let database_2_postgresql_connection_pool = match Pool::builder()
                .build(
                    PostgresqlConnectionManager::new(
                        environment_configuration.get_resource_database_2_postgresql_configuration().clone(), NoTls
                    )
                ).await {
                Ok(database_2_postgresql_connection_pool_) => database_2_postgresql_connection_pool_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            PostgresqlConnectionPoolWorkflowTypeAggregator::LocalDevelopment { database_1_postgresql_connection_pool, database_2_postgresql_connection_pool }
        };

        let redis_connection_manager = match RedisConnectionManager::new(environment_configuration.get_resource_redis_url().clone()) {
            Ok(redis_connection_manager_) => redis_connection_manager_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
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
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let builder = Server::bind(environment_configuration.get_application_server_socket_address());

        // TODO  TODO  TODO ---------  убрать Замыкания, написав и стипизировав функцию (https://docs.rs/futures/latest/futures/future/type.BoxFuture.html может помочь). Либо так https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs Но здесь сущает future::Ready<>.
        let service = make_service_fn(
            move |_: &AddrStream| {
                let environment_configuration_ = environment_configuration.clone();

                let postgresql_connection_pool_workflow_type_aggregator_ = postgresql_connection_pool_workflow_type_aggregator.clone();

                let redis_connection_pool_ = redis_connection_pool.clone();

                async move {
                    return Ok::<_, HyperError>(
                        service_fn(
                            move |requset| {
                                let environment_configuration__ = environment_configuration_.clone();

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
                                            &environment_configuration__,           // TODO TODO TODO Возможно ли как-то передать &'a environment_configuration без клонирования.
                                            requset,
                                            &database_1_postgresql_connection_pool_,
                                            &database_2_postgresql_connection_pool_,
                                            &redis_connection_pool__
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
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
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
        environment_configuration: &'a EnvironmentConfiguration,
        request: Request<Body>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>
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
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/au/cefe", &Method::POST) => {
                return application_user__authorization::check_email_for_existing::check_email_for_existing(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rbfs", &Method::POST) => {
                return application_user__authorization::register_by_first_step::register_by_first_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rbss", &Method::POST) => {
                return application_user__authorization::register_by_second_step::register_by_second_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rbls", &Method::POST) => {
                return application_user__authorization::register_by_last_step::register_by_last_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/sefr", &Method::POST) => {
                return application_user__authorization::send_email_for_register::send_email_for_register(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/abfs", &Method::POST) => {
                return application_user__authorization::authorize_by_first_step::authorize_by_first_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/abls", &Method::POST) => {
                return application_user__authorization::authorize_by_last_step::authorize_by_last_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/sefa", &Method::POST) => {
                return application_user__authorization::send_email_for_authorize::send_email_for_authorize(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rpbfs", &Method::POST) => {
                return application_user__authorization::reset_password_by_first_step::reset_password_by_first_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rpbss", &Method::POST) => {
                return application_user__authorization::reset_password_by_second_step::reset_password_by_second_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rpbls", &Method::POST) => {
                return application_user__authorization::reset_password_by_last_step::reset_password_by_last_step(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/sefrp", &Method::POST) => {
                return application_user__authorization::send_email_for_reset_password::send_email_for_reset_password(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/rauat", &Method::POST) => {
                return application_user__authorization::refresh_application_user_access_token::refresh_application_user_access_token(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // Area for existing routes with authorized user.
            ("/v1/m/au/dfod", &Method::POST) => {
                return application_user__authorization::deauthorize_from_one_device::deauthorize_from_one_device(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            ("/v1/m/au/dfad", &Method::POST) => {
                return application_user__authorization::deauthorize_from_all_devices::deauthorize_from_all_devices(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
            ("/v1/m/c/gmbn", &Method::POST) => {
                return channel__base::get_many_by_name::get_many_by_name(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
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
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
                    ("/v1/m/au/cefe_", &Method::POST) => {
                        return application_user__authorization::check_email_for_existing::check_email_for_existing_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rbfs_", &Method::POST) => {
                        return application_user__authorization::register_by_first_step::register_by_first_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rbss_", &Method::POST) => {
                        return application_user__authorization::register_by_second_step::register_by_second_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rbls_", &Method::POST) => {
                        return application_user__authorization::register_by_last_step::register_by_last_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/sefr_", &Method::POST) => {
                        return application_user__authorization::send_email_for_register::send_email_for_register_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/abfs_", &Method::POST) => {
                        return application_user__authorization::authorize_by_first_step::authorize_by_first_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/abls_", &Method::POST) => {
                        return application_user__authorization::authorize_by_last_step::authorize_by_last_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/sefa_", &Method::POST) => {
                        return application_user__authorization::send_email_for_authorize::send_email_for_authorize_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rpbfs_", &Method::POST) => {
                        return application_user__authorization::reset_password_by_first_step::reset_password_by_first_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rpbss_", &Method::POST) => {
                        return application_user__authorization::reset_password_by_second_step::reset_password_by_second_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rpbls_", &Method::POST) => {
                        return application_user__authorization::reset_password_by_last_step::reset_password_by_last_step_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/sefrp_", &Method::POST) => {
                        return application_user__authorization::send_email_for_reset_password::send_email_for_reset_password_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/rauat_", &Method::POST) => {
                        return application_user__authorization::refresh_application_user_access_token::refresh_application_user_access_token_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // Area for existing routes with authorized user.
                    ("/v1/m/au/dfod_", &Method::POST) => {
                        return application_user__authorization::deauthorize_from_one_device::deauthorize_from_one_device_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    ("/v1/m/au/dfad_", &Method::POST) => {
                        return application_user__authorization::deauthorize_from_all_devices::deauthorize_from_all_devices_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional, but POST is used. This is because there is a restriction on mobile frontend.
                    ("/v1/m/c/gmbn_", &Method::POST) => {
                        return channel__base::get_many_by_name::get_many_by_name_(
                            environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // Area for not existing routes.
                    _ => {}
                }

                return route_not_found::route_not_found(
                    environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
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