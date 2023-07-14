use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::void::ErrorVoid;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::functionality::service::creator::RedisConnectonPool;
use crate::presentation_layer::data::http_route_registry::HttpRouteRegistry;
use crate::presentation_layer::functionality::action::route_not_found;
use crate::presentation_layer::functionality::action::version_1::application_user__authorization;
use crate::presentation_layer::functionality::action::version_1::channel__base;
use crate::presentation_layer::functionality::action::version_1::channel_subscription__base;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::hyper::Method;
use extern_crate::hyper::Server;
use extern_crate::hyper::server::conn::AddrStream;
use extern_crate::tokio::select;
use extern_crate::hyper::service::make_service_fn;
use extern_crate::hyper::service::service_fn;
use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::Config as PostgresqlConfiguration;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio::runtime::Builder;
use extern_crate::tokio::signal::unix::signal;
use extern_crate::tokio::signal::unix::SignalKind;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::time::Duration;

pub struct RunServerProcessor;

impl RunServerProcessor {
    pub fn process() -> Result<(), ErrorAuditor> {
        let runtime = match Builder::new_multi_thread().enable_all().build() {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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

                        // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
    async fn run_http_server() -> Result<(), ErrorAuditor> {
        let mut application_http_socket_address_registry = match ENVIRONMENT_CONFIGURATION.application_server.tcp.socket_address.0.to_socket_addrs() {
            Ok(application_http_socket_address_registry_) => application_http_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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
                    ErrorAuditor::new(
                        BaseError::LogicError {
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
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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
            .http2_only(ENVIRONMENT_CONFIGURATION.application_server.http.http2_only)
            .http2_adaptive_window(ENVIRONMENT_CONFIGURATION.application_server.http.adaptive_window)
            .http2_initial_connection_window_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.connection_window_size))
            .http2_initial_stream_window_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.stream_window_size))
            .http2_max_concurrent_streams(u32::MAX)
            .http2_max_frame_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_frame_size))
            .http2_max_send_buf_size(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_sending_buffer_size as usize);

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive_seconds {
            Some(keepalive_seconds_) => {
                server_builder.tcp_keepalive(Some(Duration::from_secs(keepalive_seconds_)))
            }
            None => {
                server_builder.tcp_keepalive(None)
            }
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.http.keepalive {
            Some(ref keepalive_) => {
                server_builder
                    .http2_keep_alive_interval(Some(Duration::from_secs(keepalive_.interval_seconds)))
                    .http2_keep_alive_timeout(Duration::from_secs(keepalive_.timeout_seconds))
            }
            None => {
                server_builder.http2_keep_alive_interval(None)
            }
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
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
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
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
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
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::RedisError {
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

        let service = make_service_fn(
            move |_: &'_ AddrStream| -> _ {
                let postgresql_connection_pool_aggregator_ = postgresql_connection_pool_aggregator.clone();

                let database_1_redis_connection_pool_ = database_1_redis_connection_pool.clone();

                let future = async move {
                    let service_fn = service_fn(
                        move |request: Request| -> _ {
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

        let signal_interrupt_future = match signal(SignalKind::interrupt()) {
            Ok(mut signal) => {
                async move {
                    signal.recv().await;

                    ()
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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

        let signal_terminate_future = match signal(SignalKind::terminate()) {
            Ok(mut signal) => {
                async move {
                    signal.recv().await;

                    ()
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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

        let graceful_shutdown_signal = async {
            select! {
                _ = signal_interrupt_future => {},
                _ = signal_terminate_future => {},
            }
        };


        // #[cfg(unix)]
        // {

        // };
        // #[cfg(not(unix))]
        // {
        //     std::future::pending::<()>()
        // };

        if let Err(error) = server_builder.serve(service).with_graceful_shutdown(graceful_shutdown_signal).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::OtherError {
                            other_error: OtherError::new(error),
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

    async fn resolve<'a, T>(
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
        let route = request.uri().path();

        let method = request.method();

        match (
            route, method,
        ) {
            // Area for existing routes with not authorized user.
            // GET functional.
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING, &Method::POST) => {
                return application_user__authorization::check_nickname_for_existing::CheckNicknameForExisting::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING, &Method::POST) => {
                return application_user__authorization::check_email_for_existing::CheckEmailForExisting::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_FIRST_STEP, &Method::POST) => {
                return application_user__authorization::register_by_first_step::RegisterByFirstStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_SECOND_STEP, &Method::POST) => {
                return application_user__authorization::register_by_second_step::RegisterBySecondStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_LAST_STEP, &Method::POST) => {
                return application_user__authorization::register_by_last_step::RegisterByLastStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_REGISTER, &Method::POST) => {
                return application_user__authorization::send_email_for_register::SendEmailForRegister::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP, &Method::POST) => {
                return application_user__authorization::authorize_by_first_step::AuthorizeByFirstStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_LAST_STEP, &Method::POST) => {
                return application_user__authorization::authorize_by_last_step::AuthorizeByLastStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE, &Method::POST) => {
                return application_user__authorization::send_email_for_authorize::SendEmailForAuthorize::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP, &Method::POST) => {
                return application_user__authorization::reset_password_by_first_step::ResetPasswordByFirstStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP, &Method::POST) => {
                return application_user__authorization::reset_password_by_second_step::ResetPasswordBySecondStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP, &Method::POST) => {
                return application_user__authorization::reset_password_by_last_step::ResetPasswordByLastStep::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD, &Method::POST) => {
                return application_user__authorization::send_email_for_reset_password::SendEmailForResetPassword::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REFRESH_APPLICATION_USER_ACCESS_TOKEN, &Method::POST) => {
                return application_user__authorization::refresh_application_user_access_token::RefreshApplicationUserAccessToken::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // Area for existing routes with authorized user.
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE, &Method::POST) => {
                return application_user__authorization::deauthorize_from_one_device::DeauthorizeFromOneDevice::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE, &Method::POST) => {
                return application_user__authorization::deauthorize_from_all_devices::DeauthorizeFromAllDevices::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_ONE_BY_ID, &Method::POST) => {
                return channel__base::get_one_by_id::GetOneByID::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS, &Method::POST) => {
                return channel__base::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_SUBSCRIPTION, &Method::POST) => {
                return channel__base::get_many_by_subscription::GetManyBySubscription::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_PUBLIC_BY_NAME, &Method::POST) => {
                return channel__base::get_many_public_by_name::GetManyPublicByName::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            (HttpRouteRegistry::VERSION_1__CHANNEL_SUBSCRIPTION__CREATE, &Method::POST) => {
                return channel_subscription__base::create::Create::run(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
            // Area for not existing routes.
            _ => {
                #[cfg(feature = "manual_testing")]
                {
                    match (
                        route, method,
                    ) {
                        // Area for existing routes with not authorized user.
                        // GET functional.
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING_, &Method::POST) => {
                            return application_user__authorization::check_nickname_for_existing::CheckNicknameForExisting::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // GET functional.
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING_, &Method::POST) => {
                            return application_user__authorization::check_email_for_existing::CheckEmailForExisting::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_FIRST_STEP_, &Method::POST) => {
                            return application_user__authorization::register_by_first_step::RegisterByFirstStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_SECOND_STEP_, &Method::POST) => {
                            return application_user__authorization::register_by_second_step::RegisterBySecondStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_LAST_STEP_, &Method::POST) => {
                            return application_user__authorization::register_by_last_step::RegisterByLastStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_REGISTER_, &Method::POST) => {
                            return application_user__authorization::send_email_for_register::SendEmailForRegister::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP_, &Method::POST) => {
                            return application_user__authorization::authorize_by_first_step::AuthorizeByFirstStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_LAST_STEP_, &Method::POST) => {
                            return application_user__authorization::authorize_by_last_step::AuthorizeByLastStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE_, &Method::POST) => {
                            return application_user__authorization::send_email_for_authorize::SendEmailForAuthorize::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP_, &Method::POST) => {
                            return application_user__authorization::reset_password_by_first_step::ResetPasswordByFirstStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP_, &Method::POST) => {
                            return application_user__authorization::reset_password_by_second_step::ResetPasswordBySecondStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP_, &Method::POST) => {
                            return application_user__authorization::reset_password_by_last_step::ResetPasswordByLastStep::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD_, &Method::POST) => {
                            return application_user__authorization::send_email_for_reset_password::SendEmailForResetPassword::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REFRESH_APPLICATION_USER_ACCESS_TOKEN_, &Method::POST) => {
                            return application_user__authorization::refresh_application_user_access_token::RefreshApplicationUserAccessToken::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // Area for existing routes with authorized user.
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE_, &Method::POST) => {
                            return application_user__authorization::deauthorize_from_one_device::DeauthorizeFromOneDevice::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE_, &Method::POST) => {
                            return application_user__authorization::deauthorize_from_all_devices::DeauthorizeFromAllDevices::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // GET functional.
                        (HttpRouteRegistry::VERSION_1__CHANNEL__GET_ONE_BY_ID_, &Method::POST) => {
                            return channel__base::get_one_by_id::GetOneByID::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // GET functional.
                        (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_, &Method::POST) => {
                            return channel__base::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // GET functional.
                        (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_SUBSCRIPTION_, &Method::POST) => {
                            return channel__base::get_many_by_subscription::GetManyBySubscription::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // GET functional.
                        (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_PUBLIC_BY_NAME_, &Method::POST) => {
                            return channel__base::get_many_public_by_name::GetManyPublicByName::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        (HttpRouteRegistry::VERSION_1__CHANNEL_SUBSCRIPTION__CREATE_, &Method::POST) => {
                            return channel_subscription__base::create::Create::run_(
                                request,
                                database_1_postgresql_connection_pool,
                                database_2_postgresql_connection_pool,
                                database_1_redis_connection_pool,
                            )
                            .await;
                        }
                        // Area for not existing routes.
                        _ => {}
                    }
                }

                return route_not_found::route_not_found(
                    request,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                    database_1_redis_connection_pool,
                )
                .await;
            }
        }
    }
}

#[derive(Clone)]
enum PostgresqlConnectionPoolAggregator {
    LocalDevelopment {
        database_1_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
        database_2_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
    },
}
