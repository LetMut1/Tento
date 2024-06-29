use super::CommandProcessor;
pub use crate::infrastructure_layer::data::control_type::RunServer;
use crate::{
    infrastructure_layer::{
        data::{
            alternative_workflow::{
                AlternativeWorkflow,
                OptionConverter,
                ResultConverter,
            },
            auditor::Backtrace,
            control_type::{
                ApplicationUser__Authorization___AuthorizeByFirstStep,
                ApplicationUser__Authorization___AuthorizeByLastStep,
                ApplicationUser__Authorization___CheckEmailForExisting,
                ApplicationUser__Authorization___CheckNicknameForExisting,
                ApplicationUser__Authorization___DeauthorizeFromAllDevices,
                ApplicationUser__Authorization___DeauthorizeFromOneDevice,
                ApplicationUser__Authorization___RefreshAccessToken,
                ApplicationUser__Authorization___RegisterByFirstStep,
                ApplicationUser__Authorization___RegisterByLastStep,
                ApplicationUser__Authorization___RegisterBySecondStep,
                ApplicationUser__Authorization___ResetPasswordByFirstStep,
                ApplicationUser__Authorization___ResetPasswordByLastStep,
                ApplicationUser__Authorization___ResetPasswordBySecondStep,
                ApplicationUser__Authorization___SendEmailForAuthorize,
                ApplicationUser__Authorization___SendEmailForRegister,
                ApplicationUser__Authorization___SendEmailForResetPassword,
                ChannelSubscription__Base___Create,
                Channel__Base___GetManyByNameInSubscriptions,
                Channel__Base___GetManyBySubscription,
                Channel__Base___GetManyPublicByName,
                Channel__Base___GetOneById,
                PostgresqlConnectionPoolNoTls,
                Request,
                Response,
                RouteNotFound,
            },
            environment_configuration::EnvironmentConfiguration,
            void::Void,
        },
        functionality::service::{
            creator::Creator,
            loader::Loader,
            logger::Logger,
        },
    },
    presentation_layer::{
        data::action_route::{
            ActionRoute_,
            ApplicationUser__Authorization_,
            ChannelSubscription__Base_,
            Channel__Base_,
            ACTION_ROUTE,
        },
        functionality::action::Action,
    },
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use hyper::{
    server::conn::AddrStream,
    Method,
    Server,
};
use matchit::Router;
#[cfg(not(feature = "file_log"))]
use std::io::stdout;
use std::{
    clone::Clone,
    future::Future,
    marker::{
        Send,
        Sync,
    },
    net::ToSocketAddrs,
    sync::{
        Arc,
        OnceLock,
    },
    time::Duration,
};
use tokio::{
    runtime::Builder,
    signal::unix::SignalKind,
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use tracing::Level;
use tracing_appender::{
    non_blocking::{
        NonBlockingBuilder,
        WorkerGuard,
    },
    rolling::{
        RollingFileAppender,
        Rotation,
    },
};
use tracing_subscriber::FmtSubscriber;
static ENVIRONMENT_CONFIGURATION: OnceLock<EnvironmentConfiguration> = OnceLock::new();
impl CommandProcessor<RunServer> {
    const QUANTITY_OF_SECONDS_FOR_RERUN_SERVING: u64 = 1;
    pub fn process() -> Result<(), AlternativeWorkflow> {
        let environment_configuration = Self::initialize_environment()?;
        let _worker_guard = Self::initialize_logger(environment_configuration)?;
        Self::run_runtime(environment_configuration)?;
        return Ok(());
    }
    fn initialize_environment() -> Result<&'static EnvironmentConfiguration, AlternativeWorkflow> {
        let environment_configuration_file_path = format!(
            "{}/environment_configuration",
            std::env::var("CARGO_MANIFEST_DIR").into_internal_runtime(Backtrace::new(line!(), file!()))?.as_str(),
        );
        let environment_configuration = Loader::<EnvironmentConfiguration>::load_from_file(environment_configuration_file_path.as_str())?;
        match ENVIRONMENT_CONFIGURATION.get() {
            Some(_) => {
                return Err(
                    AlternativeWorkflow::new_internal_logic_value_already_exist(
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
                        AlternativeWorkflow::new_internal_logic_value_already_exist(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            }
        }
        return ENVIRONMENT_CONFIGURATION.get().into_internal_logic_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
    fn initialize_logger<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<WorkerGuard, AlternativeWorkflow> {
        let non_blocking;
        let worker_guard;
        #[cfg(feature = "file_log")]
        {
            let rolling_file_appender = RollingFileAppender::new(
                Rotation::DAILY,
                environment_configuration.logging.directory_path.as_str(),
                environment_configuration.logging.file_name_prefix.as_str(),
            );
            (
                non_blocking,
                worker_guard,
            ) = NonBlockingBuilder::default().finish(rolling_file_appender);
        }
        #[cfg(not(feature = "file_log"))]
        {
            (
                non_blocking,
                worker_guard,
            ) = NonBlockingBuilder::default().finish(stdout());
        }
        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_writer(non_blocking)
            .with_file(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();
        tracing::subscriber::set_global_default(fmt_subscriber).into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(worker_guard);
    }
    fn run_runtime(environment_configuration: &'static EnvironmentConfiguration) -> Result<(), AlternativeWorkflow> {
        if environment_configuration.tokio_runtime.maximum_blocking_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return Err(
                AlternativeWorkflow::new_internal_logic(
                    "Invalid Tokio runtime configuration.",
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
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?
            .block_on(Self::serve(environment_configuration))?;
        return Ok(());
    }
    async fn serve(environment_configuration: &'static EnvironmentConfiguration) -> Result<(), AlternativeWorkflow> {
        'a: loop {
            if let Err(alternative_workflow) = Self::serve_(environment_configuration).await {
                Logger::<AlternativeWorkflow>::log(&alternative_workflow);
                tokio::time::sleep(Duration::from_secs(Self::QUANTITY_OF_SECONDS_FOR_RERUN_SERVING)).await;
                continue 'a;
            }
            break 'a;
        }
        return Ok(());
    }
    async fn serve_(environment_configuration: &'static EnvironmentConfiguration) -> Result<(), AlternativeWorkflow> {
        let router = Self::create_router()?;
        // TODO TODO в Env
        let mut application_http_socket_address_registry = environment_configuration.application_server.tcp.socket_address.to_socket_addrs().into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let application_http_socket_address = match application_http_socket_address_registry.next() {
            Some(application_http_socket_address_) => application_http_socket_address_,
            None => {
                return Err(
                    AlternativeWorkflow::new_internal_logic(
                        "Invalid socket address.",
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };
        let signal_interrupt_future = Self::create_signal(SignalKind::interrupt())?;
        let signal_terminate_future = Self::create_signal(SignalKind::terminate())?;
        let graceful_shutdown_signal_future = async move {
            tokio::select! {
                _ = signal_interrupt_future => {
                    ()
                },
                _ = signal_terminate_future => {
                    ()
                },
            }
        };
        let mut server_builder = Server::try_bind(&application_http_socket_address).into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
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
            Some(ref keepalive_) => {
                server_builder
                    .http2_keep_alive_interval(Some(Duration::from_secs(keepalive_.interval_duration)))
                    .http2_keep_alive_timeout(Duration::from_secs(keepalive_.timeout_duration))
            }
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
        let database_1_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_1(environment_configuration).await?;
        let database_2_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_2(environment_configuration).await?;
        let router_ = Arc::new(router);
        let make_service_function_closure = move |_: &'_ AddrStream| -> _ {
            let router__ = router_.clone();
            let database_1_postgresql_connection_pool_ = database_1_postgresql_connection_pool.clone();
            let database_2_postgresql_connection_pool_ = database_2_postgresql_connection_pool.clone();
            return async move {
                return Ok::<_, Void>(
                    hyper::service::service_fn(
                        move |request: Request| -> _ {
                            let router___ = router__.clone();
                            let database_1_postgresql_connection_pool__ = database_1_postgresql_connection_pool_.clone();
                            let database_2_postgresql_connection_pool__ = database_2_postgresql_connection_pool_.clone();
                            return async move {
                                let response = Self::resolve(
                                    environment_configuration,
                                    router___,
                                    request,
                                    &database_1_postgresql_connection_pool__,
                                    &database_2_postgresql_connection_pool__,
                                )
                                .await;
                                return Ok::<_, Void>(response);
                            };
                        },
                    ),
                );
            };
        };
        server_builder
            .serve(hyper::service::make_service_fn(make_service_function_closure))
            .with_graceful_shutdown(graceful_shutdown_signal_future)
            .await
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        return Ok(());
    }
    fn create_router() -> Result<Router<ActionRoute_>, AlternativeWorkflow> {
        let mut router = Router::new();
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.check_nickname_for_existing,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.check_email_for_existing,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_first_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_second_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_last_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_register,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_first_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_last_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_authorize,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_first_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_second_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_last_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_reset_password,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.refresh_access_token,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_one_by_id,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetOneById,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_many_by_name_in_subscription,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyByNameInSubscriptions,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_many_by_subscription,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyBySubscription,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_many_piblic_by_name,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyPublicByName,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel_subscription___base.create,
                ActionRoute_::ChannelSubscription__Base {
                    channel_subscription___base: ChannelSubscription__Base_::Create,
                },
            )
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        #[cfg(feature = "manual_testing")]
        {
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.check_nickname_for_existing_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.check_email_for_existing_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.regisgter_by_first_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.regisgter_by_second_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.regisgter_by_last_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.send_email_for_register_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.authorize_by_first_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.authorize_by_last_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.send_email_for_authorize_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.reset_password_by_first_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.reset_password_by_second_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.reset_password_by_last_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.send_email_for_reset_password_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.refresh_access_token_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_one_by_id_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetOneById_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_many_by_name_in_subscription_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetManyByNameInSubscriptions_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_many_by_subscription_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetManyBySubscription_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_many_piblic_by_name_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetManyPublicByName_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel_subscription___base.create_,
                    ActionRoute_::ChannelSubscription__Base {
                        channel_subscription___base: ChannelSubscription__Base_::Create_,
                    },
                )
                .into_internal_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
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
    fn create_signal(signal_kind: SignalKind) -> Result<impl Future<Output = ()>, AlternativeWorkflow> {
        let mut signal = tokio::signal::unix::signal(signal_kind).into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let signal_ = async move {
            signal.recv().await;
            ()
        };
        return Ok(signal_);
    }
}
