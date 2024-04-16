use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Auditor;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::postgresql_connection_pool::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::functionality::service::creator::redis_connection_pool::RedisConnectonPool;
use crate::presentation_layer::data::action_route::ActionRoute_;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___AuthorizeByFirstStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___AuthorizeByLastStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___CheckEmailForExisting;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___CheckNicknameForExisting;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___DeauthorizeFromAllDevices;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___DeauthorizeFromOneDevice;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RefreshAccessToken;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterByFirstStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterByLastStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterBySecondStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___ResetPasswordByFirstStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___ResetPasswordByLastStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___ResetPasswordBySecondStep;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___SendEmailForAuthorize;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___SendEmailForRegister;
use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___SendEmailForResetPassword;
use crate::infrastructure_layer::data::control_type::Channel__Base___GetManyByNameInSubscriptions;
use crate::infrastructure_layer::data::control_type::Channel__Base___GetManyBySubscription;
use crate::infrastructure_layer::data::control_type::Channel__Base___GetManyPublicByName;
use crate::infrastructure_layer::data::control_type::Channel__Base___GetOneById;
use crate::infrastructure_layer::data::control_type::ChannelSubscription__Base___Create;
use crate::infrastructure_layer::data::control_type::RouteNotFound;
use crate::presentation_layer::data::action_route::ACTION_ROUTE;
use crate::presentation_layer::data::action_route::ApplicationUser__Authorization_;
use crate::presentation_layer::data::action_route::Channel__Base_;
use crate::presentation_layer::data::action_route::ChannelSubscription__Base_;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Method;
use hyper::Server;
use matchit::Router;
use redis::ConnectionInfo;
use std::clone::Clone;
use std::future::Future;
use std::marker::Send;
use crate::presentation_layer::functionality::action::Action;
use std::marker::Sync;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::select;
use tokio::signal::unix::signal;
use tokio::signal::unix::SignalKind;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Config as PostgresqlConfiguration;
use tokio_postgres::Socket;
use super::CommandProcessor;
use tracing_appender::rolling::Rotation;
use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::non_blocking::NonBlockingBuilder;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;

pub use crate::infrastructure_layer::data::control_type::RunServer;

impl CommandProcessor<RunServer> {
    pub fn process() -> Result<(), Auditor<Error>> {
        let _worker_guard = match Self::configure_logger() {
            Ok(worker_guard_) => worker_guard_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        if let Err(mut error) = Self::run_runtime() {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error);
        }

        return Ok(());
    }

    fn configure_logger() -> Result<WorkerGuard, Auditor<Error>> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            ENVIRONMENT_CONFIGURATION.logging.directory_path.0,
            ENVIRONMENT_CONFIGURATION.logging.file_name_prefix.0,
        );

        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(rolling_file_appender);

        let mut logger_level = Level::INFO;

        #[cfg(feature = "logger_level_trace")]
        {
            logger_level = Level::TRACE;
        }

        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(logger_level)
            .with_writer(non_blocking)
            .with_file(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();

        if let Err(error) = set_global_default(fmt_subscriber) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        return Ok(worker_guard);
    }

    fn run_runtime() -> Result<(), Auditor<Error>> {
        if ENVIRONMENT_CONFIGURATION.tokio_runtime.maximum_blocking_threads_quantity == 0
            || ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_threads_quantity == 0
            || ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Logic {
                        message: "Invalid Tokio runtime configuration.",
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let runtime = match Builder::new_multi_thread()
            .max_blocking_threads(ENVIRONMENT_CONFIGURATION.tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_threads_quantity)
            .thread_stack_size(ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_thread_stack_size)
            .enable_all()
            .build()
        {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(mut error) = runtime.block_on(Self::run_server()) {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error);
        }

        return Ok(());
    }

    async fn run_server() -> Result<(), Auditor<Error>> {
        #[derive(Clone)]
        enum PostgresqlConnectionPoolAggregator {
            LocalDevelopment {
                database_1_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
                database_2_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
            },
        }

        let router = match Self::create_router() {
            Ok(router_) => router_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let mut application_http_socket_address_registry = match ENVIRONMENT_CONFIGURATION.application_server.tcp.socket_address.0.to_socket_addrs() {
            Ok(application_http_socket_address_registry_) => application_http_socket_address_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_http_socket_address = match application_http_socket_address_registry.next() {
            Some(application_http_socket_address_) => application_http_socket_address_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Invalid socket address.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let mut server_builder = match Server::try_bind(&application_http_socket_address) {
            Ok(builder_) => builder_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        server_builder = server_builder
            .tcp_nodelay(ENVIRONMENT_CONFIGURATION.application_server.tcp.nodelay)
            .tcp_sleep_on_accept_errors(ENVIRONMENT_CONFIGURATION.application_server.tcp.sleep_on_accept_errors)
            .tcp_keepalive_retries(ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive.retries_quantity);

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive.duration {
            Some(duration) => server_builder.tcp_keepalive(Some(Duration::from_secs(duration))),
            None => server_builder.tcp_keepalive(None),
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive.interval_duration {
            Some(interval_duration) => server_builder.tcp_keepalive_interval(Some(Duration::from_secs(interval_duration))),
            None => server_builder.tcp_keepalive_interval(None),
        };

        server_builder = server_builder
            .http2_only(true)
            .http2_max_header_list_size(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_header_list_size)
            .http2_adaptive_window(ENVIRONMENT_CONFIGURATION.application_server.http.adaptive_window)
            .http2_initial_connection_window_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.connection_window_size))
            .http2_initial_stream_window_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.stream_window_size))
            .http2_max_concurrent_streams(u32::MAX)
            .http2_max_frame_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_frame_size))
            .http2_max_send_buf_size(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_sending_buffer_size as usize);

        if ENVIRONMENT_CONFIGURATION.application_server.http.enable_connect_protocol {
            server_builder = server_builder.http2_enable_connect_protocol();
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.http.keepalive {
            Some(ref keepalive_) => server_builder.http2_keep_alive_interval(Some(Duration::from_secs(keepalive_.interval_duration))).http2_keep_alive_timeout(Duration::from_secs(keepalive_.timeout_duration)),
            None => server_builder.http2_keep_alive_interval(None),
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.http.maximum_pending_accept_reset_streams {
            Some(maximum_pending_accept_reset_streams_) => server_builder.http2_max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams_)),
            None => server_builder.http2_max_pending_accept_reset_streams(None),
        };

        if let Some(ref tls) = ENVIRONMENT_CONFIGURATION.application_server.http.tls {
            todo!("// TODO ssl_protocolsTLSv1 TLSv1.1 TLSv1.2 TLSv1.3;  ssl_ciphers HIGH:!aNULL:!MD5;")
        }

        #[cfg(feature = "manual_testing")]
        {
            server_builder = server_builder.http2_only(false);
        }

        let database_1_postgresql_configuration = match PostgresqlConfiguration::from_str(ENVIRONMENT_CONFIGURATION.resource.postgresql.database_1_url.0) {
            Ok(database_1_postgresql_configuration_) => database_1_postgresql_configuration_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let database_2_postgresql_configuration = match PostgresqlConfiguration::from_str(ENVIRONMENT_CONFIGURATION.resource.postgresql.database_2_url.0) {
            Ok(database_2_postgresql_configuration_) => database_2_postgresql_configuration_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let database_1_redis_connection_info = match ConnectionInfo::from_str(ENVIRONMENT_CONFIGURATION.resource.redis.database_1_url.0) {
            Ok(database_1_redis_connection_info_) => database_1_redis_connection_info_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let postgresql_connection_pool_aggregator = match ENVIRONMENT_CONFIGURATION.environment {
            Environment::Production => {
                todo!("TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ");
            }
            Environment::Development | Environment::LocalDevelopment => {
                let database_1_postgresql_connection_pool = match Creator::<PostgresqlConnectionPoolNoTls>::create(
                    &ENVIRONMENT_CONFIGURATION.environment,
                    &database_1_postgresql_configuration,
                )
                .await
                {
                    Ok(database_1_postgresql_connection_pool_) => database_1_postgresql_connection_pool_,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        return Err(error);
                    }
                };

                let database_2_postgresql_connection_pool = match Creator::<PostgresqlConnectionPoolNoTls>::create(
                    &ENVIRONMENT_CONFIGURATION.environment,
                    &database_2_postgresql_configuration,
                )
                .await
                {
                    Ok(database_2_postgresql_connection_pool_) => database_2_postgresql_connection_pool_,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        return Err(error);
                    }
                };

                PostgresqlConnectionPoolAggregator::LocalDevelopment {
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                }
            }
        };

        let database_1_redis_connection_pool = match Creator::<RedisConnectonPool>::create(
            &ENVIRONMENT_CONFIGURATION.environment,
            &database_1_redis_connection_info,
        )
        .await
        {
            Ok(database_1_redis_connection_pool_) => database_1_redis_connection_pool_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let router_ = Arc::new(router);

        let service = make_service_fn(
            move |_: &'_ AddrStream| -> _ {
                let router__ = router_.clone();

                let postgresql_connection_pool_aggregator_ = postgresql_connection_pool_aggregator.clone();

                let database_1_redis_connection_pool_ = database_1_redis_connection_pool.clone();

                let future = async move {
                    let service_fn = service_fn(
                        move |request: Request| -> _ {
                            let router___ = router__.clone();

                            let postgresql_connection_pool_aggregator__ = postgresql_connection_pool_aggregator_.clone();

                            let database_1_redis_connection_pool__ = database_1_redis_connection_pool_.clone();

                            let (database_1_postgresql_connection_pool_, database_2_postgresql_connection_pool_) = match postgresql_connection_pool_aggregator__ {
                                PostgresqlConnectionPoolAggregator::LocalDevelopment {
                                    database_1_postgresql_connection_pool,
                                    database_2_postgresql_connection_pool,
                                } => (
                                    database_1_postgresql_connection_pool,
                                    database_2_postgresql_connection_pool,
                                ),
                            };

                            let future_ = async move {
                                let response = Self::resolve(
                                    router___,
                                    request,
                                    &database_1_postgresql_connection_pool_,
                                    &database_2_postgresql_connection_pool_,
                                    &database_1_redis_connection_pool__,
                                )
                                .await;

                                Ok::<_, Void>(response)
                            };

                            return future_;
                        },
                    );

                    Ok::<_, Void>(service_fn)
                };

                return future;
            },
        );

        let signal_interrupt_future = match Self::create_signal(SignalKind::interrupt()) {
            Ok(signal_interrupt_future_) => signal_interrupt_future_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let signal_terminate_future = match Self::create_signal(SignalKind::terminate()) {
            Ok(signal_terminate_future_) => signal_terminate_future_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let graceful_shutdown_signal_future = async {
            select! {
                _ = signal_interrupt_future => {
                    ()
                },
                _ = signal_terminate_future => {
                    ()
                },
            }
        };

        if let Err(error) = server_builder.serve(service).with_graceful_shutdown(graceful_shutdown_signal_future).await {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        return Ok(());
    }

    fn create_router() -> Result<Router<ActionRoute_>, Auditor<Error>> {
        let mut router = Router::new();

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.check_nickname_for_existing,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.check_email_for_existing,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.regisgter_by_first_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.regisgter_by_second_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.regisgter_by_last_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.send_email_for_register,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.authorize_by_first_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.authorize_by_last_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.send_email_for_authorize,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.reset_password_by_first_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.reset_password_by_second_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.reset_password_by_last_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.send_email_for_reset_password,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.refresh_access_token,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.channel___base.get_one_by_id,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetOneById,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.channel___base.get_many_by_name_in_subscription,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetManyByNameInSubscriptions,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.channel___base.get_many_by_subscription,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetManyBySubscription,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.channel___base.get_many_piblic_by_name,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetManyPublicByName,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.channel_subscription___base.create,
            ActionRoute_::ChannelSubscription__Base {
                channel_subscription___base: ChannelSubscription__Base_::Create,
            },
        ) {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        #[cfg(feature = "manual_testing")]
        {
            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.check_nickname_for_existing_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.check_email_for_existing_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_first_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_second_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_last_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_register_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_first_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_last_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_authorize_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_first_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_second_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_last_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_reset_password_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.refresh_access_token_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.channel___base.get_one_by_id_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetOneById_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.channel___base.get_many_by_name_in_subscription_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyByNameInSubscriptions_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.channel___base.get_many_by_subscription_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyBySubscription_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.channel___base.get_many_piblic_by_name_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyPublicByName_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.channel_subscription___base.create_,
                ActionRoute_::ChannelSubscription__Base {
                    channel_subscription___base: ChannelSubscription__Base_::Create_,
                },
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        }

        return Ok(router);
    }

    async fn resolve<'a, T>(
        router: Arc<Router<ActionRoute_>>,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let (parts, mut body) = request.into_parts();

        let r#match = match router.at(parts.uri.path()) {
            Ok(r#match_) => r#match_,
            Err(_) => {
                return Action::<RouteNotFound>::run(&parts);
            }
        };

        match r#match.value {
            &ActionRoute_::ApplicationUser__Authorization {
                ref application_user___authorization,
            } => {
                match (
                    application_user___authorization,
                    &parts.method,
                ) {
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&ApplicationUser__Authorization_::CheckNicknameForExisting, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___CheckNicknameForExisting>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&ApplicationUser__Authorization_::CheckEmailForExisting, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___CheckEmailForExisting>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterByFirstStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RegisterByFirstStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterBySecondStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RegisterBySecondStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterByLastStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RegisterByLastStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForRegister, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___SendEmailForRegister>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::AuthorizeByFirstStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___AuthorizeByFirstStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::AuthorizeByLastStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___AuthorizeByLastStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForAuthorize, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___SendEmailForAuthorize>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordByFirstStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___ResetPasswordByFirstStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordBySecondStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___ResetPasswordBySecondStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordByLastStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___ResetPasswordByLastStep>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForResetPassword, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___SendEmailForResetPassword>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RefreshAccessToken, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RefreshAccessToken>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___DeauthorizeFromOneDevice>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___DeauthorizeFromAllDevices>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    _ => {
                        #[cfg(feature = "manual_testing")]
                        {
                            match (
                                application_user___authorization,
                                &parts.method,
                            ) {
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&ApplicationUser__Authorization_::CheckNicknameForExisting_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___CheckNicknameForExisting>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&ApplicationUser__Authorization_::CheckEmailForExisting_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___CheckEmailForExisting>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterByFirstStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RegisterByFirstStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterBySecondStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RegisterBySecondStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterByLastStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RegisterByLastStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForRegister_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___SendEmailForRegister>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::AuthorizeByFirstStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___AuthorizeByFirstStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::AuthorizeByLastStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___AuthorizeByLastStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForAuthorize_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___SendEmailForAuthorize>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordByFirstStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___ResetPasswordByFirstStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordBySecondStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___ResetPasswordBySecondStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordByLastStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___ResetPasswordByLastStep>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForResetPassword_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___SendEmailForResetPassword>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RefreshAccessToken_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RefreshAccessToken>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___DeauthorizeFromOneDevice>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___DeauthorizeFromAllDevices>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            &ActionRoute_::Channel__Base {
                ref channel___base,
            } => {
                match (
                    channel___base,
                    &parts.method,
                ) {
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetOneById, &Method::POST) => {
                        return Action::<Channel__Base___GetOneById>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetManyByNameInSubscriptions, &Method::POST) => {
                        return Action::<Channel__Base___GetManyByNameInSubscriptions>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetManyBySubscription, &Method::POST) => {
                        return Action::<Channel__Base___GetManyBySubscription>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetManyPublicByName, &Method::POST) => {
                        return Action::<Channel__Base___GetManyPublicByName>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    _ => {
                        #[cfg(feature = "manual_testing")]
                        {
                            match (
                                channel___base,
                                &parts.method,
                            ) {
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetOneById_, &Method::POST) => {
                                    return Action::<Channel__Base___GetOneById>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetManyByNameInSubscriptions_, &Method::POST) => {
                                    return Action::<Channel__Base___GetManyByNameInSubscriptions>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetManyBySubscription_, &Method::POST) => {
                                    return Action::<Channel__Base___GetManyBySubscription>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetManyPublicByName_, &Method::POST) => {
                                    return Action::<Channel__Base___GetManyPublicByName>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            &ActionRoute_::ChannelSubscription__Base {
                ref channel_subscription___base,
            } => {
                match (
                    channel_subscription___base,
                    &parts.method,
                ) {
                    (&ChannelSubscription__Base_::Create, &Method::POST) => {
                        return Action::<ChannelSubscription__Base___Create>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    _ => {
                        #[cfg(feature = "manual_testing")]
                        {
                            match (
                                channel_subscription___base,
                                &parts.method,
                            ) {
                                (&ChannelSubscription__Base_::Create_, &Method::POST) => {
                                    return Action::<ChannelSubscription__Base___Create>::run_(
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        return Action::<RouteNotFound>::run(&parts);
    }

    fn create_signal(signal_kind: SignalKind) -> Result<impl Future<Output = ()>, Auditor<Error>> {
        let signal = match signal(signal_kind) {
            Ok(mut signal) => {
                async move {
                    signal.recv().await;

                    ()
                }
            }
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(signal);
    }
}
