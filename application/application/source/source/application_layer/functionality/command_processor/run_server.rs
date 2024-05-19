use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::postgresql_connection_pool::PostgresqlConnectionPoolNoTls;
use crate::presentation_layer::data::action_route::ActionRoute_;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
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
use hyper::Method;
use hyper::Server;
use matchit::Router;
use std::clone::Clone;
use std::future::Future;
use std::marker::Send;
use crate::presentation_layer::functionality::action::Action;
use std::marker::Sync;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::signal::unix::SignalKind;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use super::CommandProcessor;
use tracing_appender::rolling::Rotation;
use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::non_blocking::NonBlockingBuilder;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::loader::Loader;

pub use crate::infrastructure_layer::data::control_type::RunServer;

static ENVIRONMENT_CONFIGURATION: OnceLock<EnvironmentConfiguration> = OnceLock::new();

impl CommandProcessor<RunServer> {
    pub fn process() -> Result<(), Auditor<Error>> {
        let environment_configuration = Self:: initialize_environment()?;

        let _worker_guard = Self::initialize_logger(environment_configuration)?;

        Self::run_runtime(environment_configuration)?;

        return Ok(());
    }

    fn initialize_environment() -> Result<&'static EnvironmentConfiguration, Auditor<Error>> {
        let environment_configuration_file_path = format!(
            "{}/environment_configuration",
            std::env::var("CARGO_MANIFEST_DIR").convert(Backtrace::new(line!(), file!()))?.as_str(),
        );

        let environment_configuration = Loader::<EnvironmentConfiguration>::load_from_file(environment_configuration_file_path.as_str())?;

        match ENVIRONMENT_CONFIGURATION.get() {
            Some(_) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::new_logic_value_already_exist(),
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            None => {
                if let Err(_) = ENVIRONMENT_CONFIGURATION.set(environment_configuration) {
                    return Err(
                        Auditor::<Error>::new(
                            Error::new_logic_value_already_exist(),
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            }
        }

        return Ok(ENVIRONMENT_CONFIGURATION.get().convert_value_does_not_exist(Backtrace::new(line!(),file!()))?);
    }

    fn initialize_logger<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<WorkerGuard, Auditor<Error>> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            environment_configuration.logging.directory_path.as_str(),
            environment_configuration.logging.file_name_prefix.as_str(),
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

        tracing::subscriber::set_global_default(fmt_subscriber).convert(Backtrace::new(line!(), file!()))?;

        return Ok(worker_guard);
    }

    fn run_runtime(environment_configuration: &'static EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        if environment_configuration.tokio_runtime.maximum_blocking_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Logic {
                        message: "Invalid Tokio runtime configuration.",
                    },
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        Builder::new_multi_thread()
            .max_blocking_threads(environment_configuration.tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(environment_configuration.tokio_runtime.worker_threads_quantity)
            .thread_stack_size(environment_configuration.tokio_runtime.worker_thread_stack_size)
            .enable_all()
            .build()
            .convert(Backtrace::new(line!(), file!()))?
            .block_on(Self::run_server(environment_configuration))?;

        return Ok(());
    }

    async fn run_server(environment_configuration: &'static EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        #[derive(Clone)]
        enum PostgresqlConnectionPoolAggregator {
            LocalDevelopment {
                database_1_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
                database_2_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
            },
        }

        let router = Self::create_router()?;

        // TODO TODO в Env
        let mut application_http_socket_address_registry = environment_configuration.application_server.tcp.socket_address.to_socket_addrs().convert(Backtrace::new(line!(), file!()))?;

        let application_http_socket_address = match application_http_socket_address_registry.next() {
            Some(application_http_socket_address_) => application_http_socket_address_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Invalid socket address.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let mut server_builder = Server::try_bind(&application_http_socket_address).convert(Backtrace::new(line!(), file!()))?;

        server_builder = server_builder
            .tcp_nodelay(environment_configuration.application_server.tcp.nodelay)
            .tcp_sleep_on_accept_errors(environment_configuration.application_server.tcp.sleep_on_accept_errors)
            .tcp_keepalive_retries(environment_configuration.application_server.tcp.keepalive.retries_quantity);

        server_builder = match environment_configuration.application_server.tcp.keepalive.duration {
            Some(duration) => server_builder.tcp_keepalive(Some(Duration::from_secs(duration))),
            None => server_builder.tcp_keepalive(None),
        };

        server_builder = match environment_configuration.application_server.tcp.keepalive.interval_duration {
            Some(interval_duration) => server_builder.tcp_keepalive_interval(Some(Duration::from_secs(interval_duration))),
            None => server_builder.tcp_keepalive_interval(None),
        };

        server_builder = server_builder
            .http2_only(true)
            .http2_max_header_list_size(environment_configuration.application_server.http.maximum_header_list_size)
            .http2_adaptive_window(environment_configuration.application_server.http.adaptive_window)
            .http2_initial_connection_window_size(Some(environment_configuration.application_server.http.connection_window_size))
            .http2_initial_stream_window_size(Some(environment_configuration.application_server.http.stream_window_size))
            .http2_max_concurrent_streams(u32::MAX)
            .http2_max_frame_size(Some(environment_configuration.application_server.http.maximum_frame_size))
            .http2_max_send_buf_size(environment_configuration.application_server.http.maximum_sending_buffer_size as usize);

        if environment_configuration.application_server.http.enable_connect_protocol {
            server_builder = server_builder.http2_enable_connect_protocol();
        };

        server_builder = match environment_configuration.application_server.http.keepalive {
            Some(ref keepalive_) => server_builder.http2_keep_alive_interval(Some(Duration::from_secs(keepalive_.interval_duration))).http2_keep_alive_timeout(Duration::from_secs(keepalive_.timeout_duration)),
            None => server_builder.http2_keep_alive_interval(None),
        };

        server_builder = match environment_configuration.application_server.http.maximum_pending_accept_reset_streams {
            Some(maximum_pending_accept_reset_streams_) => server_builder.http2_max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams_)),
            None => server_builder.http2_max_pending_accept_reset_streams(None),
        };

        if let Some(ref _tls) = environment_configuration.application_server.http.tls {
            todo!("// TODO ssl_protocolsTLSv1 TLSv1.1 TLSv1.2 TLSv1.3;  ssl_ciphers HIGH:!aNULL:!MD5;")
        }

        #[cfg(feature = "manual_testing")]
        {
            server_builder = server_builder.http2_only(false);
        }

        let postgresql_connection_pool_aggregator = match environment_configuration.environment {
            Environment::Production => {
                todo!("TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ");
            }
            Environment::Development | Environment::LocalDevelopment => {
                let database_1_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_1(
                    environment_configuration,
                )
                .await?;

                let database_2_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_2(
                    environment_configuration,
                )
                .await?;

                PostgresqlConnectionPoolAggregator::LocalDevelopment {
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                }
            }
        };

        let router_ = Arc::new(router);

        let service = hyper::service::make_service_fn(
            move |_: &'_ AddrStream| -> _ {
                let router__ = router_.clone();

                let postgresql_connection_pool_aggregator_ = postgresql_connection_pool_aggregator.clone();

                let future = async move {
                    let service_fn = hyper::service::service_fn(
                        move |request: Request| -> _ {
                            let router___ = router__.clone();

                            let postgresql_connection_pool_aggregator__ = postgresql_connection_pool_aggregator_.clone();

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
                                    environment_configuration,
                                    router___,
                                    request,
                                    &database_1_postgresql_connection_pool_,
                                    &database_2_postgresql_connection_pool_,
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

        let signal_interrupt_future = Self::create_signal(SignalKind::interrupt())?;

        let signal_terminate_future = Self::create_signal(SignalKind::terminate())?;

        let graceful_shutdown_signal_future = async {
            tokio::select! {
                _ = signal_interrupt_future => {
                    ()
                },
                _ = signal_terminate_future => {
                    ()
                },
            }
        };

        server_builder.serve(service).with_graceful_shutdown(graceful_shutdown_signal_future).await.convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    fn create_router() -> Result<Router<ActionRoute_>, Auditor<Error>> {
        let mut router = Router::new();

        router.insert(
            ACTION_ROUTE.application_user___authorization.check_nickname_for_existing,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.check_email_for_existing,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.regisgter_by_first_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.regisgter_by_second_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.regisgter_by_last_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.send_email_for_register,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.authorize_by_first_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.authorize_by_last_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.send_email_for_authorize,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.reset_password_by_first_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.reset_password_by_second_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.reset_password_by_last_step,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.send_email_for_reset_password,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.refresh_access_token,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices,
            ActionRoute_::ApplicationUser__Authorization {
                application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.channel___base.get_one_by_id,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetOneById,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.channel___base.get_many_by_name_in_subscription,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetManyByNameInSubscriptions,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.channel___base.get_many_by_subscription,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetManyBySubscription,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.channel___base.get_many_piblic_by_name,
            ActionRoute_::Channel__Base {
                channel___base: Channel__Base_::GetManyPublicByName,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        router.insert(
            ACTION_ROUTE.channel_subscription___base.create,
            ActionRoute_::ChannelSubscription__Base {
                channel_subscription___base: ChannelSubscription__Base_::Create,
            },
        )
        .convert(Backtrace::new(line!(), file!()))?;

        #[cfg(feature = "manual_testing")]
        {
             router.insert(
                ACTION_ROUTE.application_user___authorization.check_nickname_for_existing_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.check_email_for_existing_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_first_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_second_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_last_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_register_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_first_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_last_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_authorize_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_first_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_second_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_last_step_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_reset_password_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.refresh_access_token_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices_,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.channel___base.get_one_by_id_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetOneById_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.channel___base.get_many_by_name_in_subscription_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyByNameInSubscriptions_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.channel___base.get_many_by_subscription_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyBySubscription_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.channel___base.get_many_piblic_by_name_,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyPublicByName_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;

             router.insert(
                ACTION_ROUTE.channel_subscription___base.create_,
                ActionRoute_::ChannelSubscription__Base {
                    channel_subscription___base: ChannelSubscription__Base_::Create_,
                },
            )
.convert(Backtrace::new(line!(), file!()))?;
        }

        return Ok(router);
    }

    async fn resolve<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        router: Arc<Router<ActionRoute_>>,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
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
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&ApplicationUser__Authorization_::CheckEmailForExisting, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___CheckEmailForExisting>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterByFirstStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RegisterByFirstStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterBySecondStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RegisterBySecondStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RegisterByLastStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RegisterByLastStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForRegister, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___SendEmailForRegister>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::AuthorizeByFirstStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___AuthorizeByFirstStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::AuthorizeByLastStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___AuthorizeByLastStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForAuthorize, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___SendEmailForAuthorize>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordByFirstStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___ResetPasswordByFirstStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordBySecondStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___ResetPasswordBySecondStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::ResetPasswordByLastStep, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___ResetPasswordByLastStep>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::SendEmailForResetPassword, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___SendEmailForResetPassword>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::RefreshAccessToken, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___RefreshAccessToken>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___DeauthorizeFromOneDevice>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices, &Method::POST) => {
                        return Action::<ApplicationUser__Authorization___DeauthorizeFromAllDevices>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
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
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&ApplicationUser__Authorization_::CheckEmailForExisting_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___CheckEmailForExisting>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterByFirstStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RegisterByFirstStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterBySecondStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RegisterBySecondStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RegisterByLastStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RegisterByLastStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForRegister_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___SendEmailForRegister>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::AuthorizeByFirstStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___AuthorizeByFirstStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::AuthorizeByLastStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___AuthorizeByLastStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForAuthorize_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___SendEmailForAuthorize>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordByFirstStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___ResetPasswordByFirstStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordBySecondStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___ResetPasswordBySecondStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::ResetPasswordByLastStep_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___ResetPasswordByLastStep>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::SendEmailForResetPassword_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___SendEmailForResetPassword>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::RefreshAccessToken_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___RefreshAccessToken>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___DeauthorizeFromOneDevice>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices_, &Method::POST) => {
                                    return Action::<ApplicationUser__Authorization___DeauthorizeFromAllDevices>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
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
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetManyByNameInSubscriptions, &Method::POST) => {
                        return Action::<Channel__Base___GetManyByNameInSubscriptions>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetManyBySubscription, &Method::POST) => {
                        return Action::<Channel__Base___GetManyBySubscription>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
                        )
                        .await;
                    }
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    (&Channel__Base_::GetManyPublicByName, &Method::POST) => {
                        return Action::<Channel__Base___GetManyPublicByName>::run(
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
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
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetManyByNameInSubscriptions_, &Method::POST) => {
                                    return Action::<Channel__Base___GetManyByNameInSubscriptions>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetManyBySubscription_, &Method::POST) => {
                                    return Action::<Channel__Base___GetManyBySubscription>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
                                    )
                                    .await;
                                }
                                // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                (&Channel__Base_::GetManyPublicByName_, &Method::POST) => {
                                    return Action::<Channel__Base___GetManyPublicByName>::run_(
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
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
                            environment_configuration,
                            &mut body,
                            &parts,
                            &r#match.params,
                            database_1_postgresql_connection_pool,
                            database_2_postgresql_connection_pool,
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
                                        environment_configuration,
                                        &mut body,
                                        &parts,
                                        &r#match.params,
                                        database_1_postgresql_connection_pool,
                                        database_2_postgresql_connection_pool,
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
        let mut signal = tokio::signal::unix::signal(signal_kind).convert(Backtrace::new(line!(), file!()))?;

        let signal_ = async move {
            signal.recv().await;

            ()
        };

        return Ok(signal_);
    }
}
