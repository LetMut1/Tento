use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::void::ErrorVoid;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::functionality::service::creator::RedisConnectonPool;
use crate::presentation_layer::data::action_route::ActionRoute_;
use crate::presentation_layer::data::action_route::ApplicationUser__Authorization_;
use crate::presentation_layer::data::action_route::ChannelSubscription__Base_;
use crate::presentation_layer::data::action_route::Channel__Base_;
use crate::presentation_layer::data::action_route::ACTION_ROUTE;
use crate::presentation_layer::functionality::action::application_user___authorization;
use crate::presentation_layer::functionality::action::channel___base;
use crate::presentation_layer::functionality::action::channel_subscription___base;
use crate::presentation_layer::functionality::action::route_not_found::RouteNotFound;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
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

pub struct RunServer;

impl RunServer {
    pub fn process() -> Result<(), ErrorAuditor_> {
        let runtime = match Builder::new_multi_thread().enable_all().build() {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(mut error) = runtime.block_on(Self::run_http_server()) {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }

        return Ok(());
    }

    async fn run_http_server() -> Result<(), ErrorAuditor_> {
        let router = match Self::create_router() {
            Ok(router_) => router_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let mut application_http_socket_address_registry = match ENVIRONMENT_CONFIGURATION.application_server.tcp.socket_address.0.to_socket_addrs() {
            Ok(application_http_socket_address_registry_) => application_http_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let application_http_socket_address = match application_http_socket_address_registry.next() {
            Some(application_http_socket_address_) => application_http_socket_address_,
            None => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Logic {
                            message: "Invalid socket address.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let mut server_builder = match Server::try_bind(&application_http_socket_address) {
            Ok(builder_) => builder_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_2_postgresql_configuration = match PostgresqlConfiguration::from_str(ENVIRONMENT_CONFIGURATION.resource.postgresql.database_2_url.0) {
            Ok(database_2_postgresql_configuration_) => database_2_postgresql_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_1_redis_connection_info = match ConnectionInfo::from_str(ENVIRONMENT_CONFIGURATION.resource.redis.database_1_url.0) {
            Ok(database_1_redis_connection_info_) => database_1_redis_connection_info_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Redis {
                                    redis_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                                None,
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
                                None,
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
                        None,
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

                                Ok::<_, ErrorVoid>(response)
                            };

                            return future_;
                        },
                    );

                    Ok::<_, ErrorVoid>(service_fn)
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
                        None,
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
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let graceful_shutdown_signal = async {
            select! {
                _ = signal_interrupt_future => {
                    ()
                },
                _ = signal_terminate_future => {
                    ()
                },
            }
        };

        if let Err(error) = server_builder.serve(service).with_graceful_shutdown(graceful_shutdown_signal).await {
            return Err(
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        }

        return Ok(());
    }

    fn create_router() -> Result<Router<ActionRoute_>, ErrorAuditor_> {
        let mut router = Router::new();

        if let Err(error) = router.insert(
            ACTION_ROUTE.application_user___authorization.check_nickname_for_existing,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting,
            },
        ) {
            return Err(
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        }

        if let Err(error) = router.insert(
            ACTION_ROUTE.channel___base.get_one_by_id,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetOneByID,
            },
        ) {
            return Err(
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }

            if let Err(error) = router.insert(
                ACTION_ROUTE.channel___base.get_one_by_id_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetOneByID_,
                },
            ) {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
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
        let r#match = match router.at(request.uri().path()) {
            Ok(r#match_) => r#match_,
            Err(_) => {
                return RouteNotFound::run(
                    request,
                    database_2_postgresql_connection_pool,
                )
                .await;
            }
        };

        let method = request.method();

        match r#match.value {
            &ActionRoute_::ApplicationUser__Authorization {
                ref application_user___authorization,
            } => {
                match (
                    application_user___authorization,
                    method,
                ) {
                    // GET functional.
                    (&ApplicationUser__Authorization_::CheckNicknameForExisting, &Method::POST) => {
                        return application_user___authorization::check_nickname_for_existing::CheckNicknameForExisting::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // GET functional.
                    (&ApplicationUser__Authorization_::CheckEmailForExisting, &Method::POST) => {
                        return application_user___authorization::check_email_for_existing::CheckEmailForExisting::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterByFirstStep, &Method::POST) => {
                        return application_user___authorization::register_by_first_step::RegisterByFirstStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterBySecondStep, &Method::POST) => {
                        return application_user___authorization::register_by_second_step::RegisterBySecondStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterByLastStep, &Method::POST) => {
                        return application_user___authorization::register_by_last_step::RegisterByLastStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForRegister, &Method::POST) => {
                        return application_user___authorization::send_email_for_register::SendEmailForRegister::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::AuthorizeByFirstStep, &Method::POST) => {
                        return application_user___authorization::authorize_by_first_step::AuthorizeByFirstStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::AuthorizeByLastStep, &Method::POST) => {
                        return application_user___authorization::authorize_by_last_step::AuthorizeByLastStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForAuthorize, &Method::POST) => {
                        return application_user___authorization::send_email_for_authorize::SendEmailForAuthorize::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordByFirstStep, &Method::POST) => {
                        return application_user___authorization::reset_password_by_first_step::ResetPasswordByFirstStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordBySecondStep, &Method::POST) => {
                        return application_user___authorization::reset_password_by_second_step::ResetPasswordBySecondStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordByLastStep, &Method::POST) => {
                        return application_user___authorization::reset_password_by_last_step::ResetPasswordByLastStep::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForResetPassword, &Method::POST) => {
                        return application_user___authorization::send_email_for_reset_password::SendEmailForResetPassword::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RefreshAccessToken, &Method::POST) => {
                        return application_user___authorization::refresh_access_token::RefreshAccessToken::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice, &Method::POST) => {
                        return application_user___authorization::deauthorize_from_one_device::DeauthorizeFromOneDevice::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices, &Method::POST) => {
                        return application_user___authorization::deauthorize_from_all_devices::DeauthorizeFromAllDevices::run(
                            request,
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
                                method,
                            ) {
                                // GET functional.
                                (&ApplicationUser__Authorization_::CheckNicknameForExisting_, &Method::POST) => {
                                    return application_user___authorization::check_nickname_for_existing::CheckNicknameForExisting::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // GET functional.
                                (&ApplicationUser__Authorization_::CheckEmailForExisting_, &Method::POST) => {
                                    return application_user___authorization::check_email_for_existing::CheckEmailForExisting::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterByFirstStep_, &Method::POST) => {
                                    return application_user___authorization::register_by_first_step::RegisterByFirstStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterBySecondStep_, &Method::POST) => {
                                    return application_user___authorization::register_by_second_step::RegisterBySecondStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterByLastStep_, &Method::POST) => {
                                    return application_user___authorization::register_by_last_step::RegisterByLastStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForRegister_, &Method::POST) => {
                                    return application_user___authorization::send_email_for_register::SendEmailForRegister::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::AuthorizeByFirstStep_, &Method::POST) => {
                                    return application_user___authorization::authorize_by_first_step::AuthorizeByFirstStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::AuthorizeByLastStep_, &Method::POST) => {
                                    return application_user___authorization::authorize_by_last_step::AuthorizeByLastStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForAuthorize_, &Method::POST) => {
                                    return application_user___authorization::send_email_for_authorize::SendEmailForAuthorize::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordByFirstStep_, &Method::POST) => {
                                    return application_user___authorization::reset_password_by_first_step::ResetPasswordByFirstStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordBySecondStep_, &Method::POST) => {
                                    return application_user___authorization::reset_password_by_second_step::ResetPasswordBySecondStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordByLastStep_, &Method::POST) => {
                                    return application_user___authorization::reset_password_by_last_step::ResetPasswordByLastStep::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForResetPassword_, &Method::POST) => {
                                    return application_user___authorization::send_email_for_reset_password::SendEmailForResetPassword::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RefreshAccessToken_, &Method::POST) => {
                                    return application_user___authorization::refresh_access_token::RefreshAccessToken::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice_, &Method::POST) => {
                                    return application_user___authorization::deauthorize_from_one_device::DeauthorizeFromOneDevice::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices_, &Method::POST) => {
                                    return application_user___authorization::deauthorize_from_all_devices::DeauthorizeFromAllDevices::run_(
                                        request,
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
                    method,
                ) {
                    // GET functional.
                    (&Channel__Base_::GetOneByID, &Method::POST) => {
                        return channel___base::get_one_by_id::GetOneByID::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // GET functional.
                    (&Channel__Base_::GetManyByNameInSubscriptions, &Method::POST) => {
                        return channel___base::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // GET functional.
                    (&Channel__Base_::GetManyBySubscription, &Method::POST) => {
                        return channel___base::get_many_by_subscription::GetManyBySubscription::run(
                            request,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                            database_1_redis_connection_pool,
                        )
                        .await;
                    }
                    // GET functional.
                    (&Channel__Base_::GetManyPublicByName, &Method::POST) => {
                        return channel___base::get_many_public_by_name::GetManyPublicByName::run(
                            request,
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
                                method,
                            ) {
                                // GET functional.
                                (&Channel__Base_::GetOneByID_, &Method::POST) => {
                                    return channel___base::get_one_by_id::GetOneByID::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // GET functional.
                                (&Channel__Base_::GetManyByNameInSubscriptions_, &Method::POST) => {
                                    return channel___base::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // GET functional.
                                (&Channel__Base_::GetManyBySubscription_, &Method::POST) => {
                                    return channel___base::get_many_by_subscription::GetManyBySubscription::run_(
                                        request,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                        database_1_redis_connection_pool,
                                    )
                                    .await;
                                }
                                // GET functional.
                                (&Channel__Base_::GetManyPublicByName_, &Method::POST) => {
                                    return channel___base::get_many_public_by_name::GetManyPublicByName::run_(
                                        request,
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
                    method,
                ) {
                    (&ChannelSubscription__Base_::Create, &Method::POST) => {
                        return channel_subscription___base::create::Create::run(
                            request,
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
                                method,
                            ) {
                                (&ChannelSubscription__Base_::Create_, &Method::POST) => {
                                    return channel_subscription___base::create::Create::run_(
                                        request,
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

        return RouteNotFound::run(
            request,
            database_2_postgresql_connection_pool,
        )
        .await;
    }

    fn create_signal(signal_kind: SignalKind) -> Result<impl Future<Output = ()>, ErrorAuditor_> {
        let signal = match signal(signal_kind) {
            Ok(mut signal) => {
                async move {
                    signal.recv().await;

                    ()
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(signal);
    }
}

#[derive(Clone)]
enum PostgresqlConnectionPoolAggregator {
    LocalDevelopment {
        database_1_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
        database_2_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
    },
}
