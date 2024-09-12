use super::CommandProcessor;
use crate::{
    application_layer::functionality::action_processor::Inner as ActionProcessorInner,
    infrastructure_layer::{
        data::{
            capture::Capture,
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
                RunServer, TokioNonBlockingTask,
            },
            environment_configuration::environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::{
            creator::Creator,
            loader::Loader,
            logger::Logger, spawner::Spawner,
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
        functionality::action::{
            Action,
            Inner as ActionInner,
        },
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
    OptionConverter,
    ResultConverter,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use matchit::Router;
use std::{
    future::Future, sync::{
        atomic::Ordering, Arc, OnceLock
    }, time::Duration
};
use tokio::{
    runtime::Builder as RuntimeBuilder,
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
use tracing_appender::non_blocking::{
    NonBlocking,
    NonBlockingBuilder,
    WorkerGuard,
};
#[cfg(feature = "file_log")]
use tracing_appender::rolling::{
    RollingFileAppender,
    Rotation,
};
use tracing_subscriber::FmtSubscriber;
use void::Void;
use hyper_x::server::conn::http2::Builder as Http2Builder;
use hyper_x::service::service_fn;
use hyper_x::Method;
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use hyper_util::rt::tokio::TokioExecutor;
use std::sync::atomic::AtomicU64;
static CONNECTION_QUANTITY: AtomicU64 = AtomicU64::new(0);
static ENVIRONMENT_CONFIGURATION: OnceLock<EnvironmentConfiguration> = OnceLock::new();
impl CommandProcessor<RunServer> {
    pub fn process() -> Result<(), AggregateError> {
        let _worker_guard;
        let environment_configuration = Self::initialize_environment()?;
        #[cfg(feature = "file_log")]
        {
            _worker_guard = Self::initialize_file_logger(environment_configuration)?;
        }
        #[cfg(not(feature = "file_log"))]
        {
            _worker_guard = Self::initialize_stdout_logger();
        }
        Self::run_runtime(environment_configuration)?;
        return Ok(());
    }
    fn initialize_environment() -> Result<&'static EnvironmentConfiguration, AggregateError> {
        let environment_configuration_file_path = format!(
            "{}/environment_configuration",
            std::env::var("CARGO_MANIFEST_DIR").into_runtime(Backtrace::new(line!(), file!()))?.as_str(),
        );
        let environment_configuration = Loader::<EnvironmentConfiguration>::load_from_file(environment_configuration_file_path.as_str())?;
        return match ENVIRONMENT_CONFIGURATION.get() {
            Some(environment_configuration__) => Ok(environment_configuration__),
            None => {
                if let Err(_) = ENVIRONMENT_CONFIGURATION.set(environment_configuration) {
                    return Err(
                        AggregateError::new_logic_(
                            Common::ValueAlreadyExist,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                ENVIRONMENT_CONFIGURATION.get().into_logic_value_does_not_exist(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )
            }
        };
    }
    #[cfg(feature = "file_log")]
    fn initialize_file_logger<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<WorkerGuard, AggregateError> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            environment_configuration.logging.directory_path.as_str(),
            environment_configuration.logging.file_name_prefix.as_str(),
        );
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(rolling_file_appender);
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Ok(worker_guard);
    }
    #[cfg(not(feature = "file_log"))]
    fn initialize_stdout_logger() -> Result<WorkerGuard, AggregateError> {
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(std::io::stdout());
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Ok(worker_guard);
    }
    fn initialize_tracing_subscriber<'a>(non_blocking: NonBlocking) -> Result<(), AggregateError> {
        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_writer(non_blocking)
            .with_file(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();
        tracing::subscriber::set_global_default(fmt_subscriber).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(());
    }
    fn run_runtime(environment_configuration: &'static EnvironmentConfiguration) -> Result<(), AggregateError> {
        if environment_configuration.tokio_runtime.maximum_blocking_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return Err(
                AggregateError::new_logic(
                    "Invalid Tokio runtime configuration.".into(),
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        RuntimeBuilder::new_multi_thread()
            .max_blocking_threads(environment_configuration.tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(environment_configuration.tokio_runtime.worker_threads_quantity)
            .thread_stack_size(environment_configuration.tokio_runtime.worker_thread_stack_size)
            .enable_all()
            .build()
            .into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?
            .block_on(Self::serve(environment_configuration))?;
        return Ok(());
    }
    fn serve(environment_configuration: &'static EnvironmentConfiguration) -> impl Future<Output = Result<(), AggregateError>> + Send {
        return async move {
            'a: loop {
                if let Err(aggregate_error) = Self::serve_(environment_configuration).await {
                    Logger::<AggregateError>::log(&aggregate_error);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue 'a;
                }
                break 'a;
            }
            return Ok(());
        };
    }
    fn serve_(environment_configuration: &'static EnvironmentConfiguration) -> impl Future<Output = Result<(), AggregateError>> + Send {
        return async move {
            let router = Self::create_router()?;
            let database_1_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_1(environment_configuration).await?;
            let database_2_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_2(environment_configuration).await?;
            let cloned = Arc::new(
                Cloned {
                    router,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                },
            );
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
            let mut http2_builder = Http2Builder::new(TokioExecutor::new())
            .max_local_error_reset_streams(Some(1024));
            http2_builder
                .auto_date_header(false)
                .max_header_list_size(environment_configuration.application_server.http.maximum_header_list_size)
                .adaptive_window(environment_configuration.application_server.http.adaptive_window)
                .initial_connection_window_size(Some(environment_configuration.application_server.http.connection_window_size))
                .initial_stream_window_size(Some(environment_configuration.application_server.http.stream_window_size))
                .max_concurrent_streams(None)
                .max_frame_size(Some(environment_configuration.application_server.http.maximum_frame_size))
                .max_send_buf_size(environment_configuration.application_server.http.maximum_sending_buffer_size as usize);
            if environment_configuration.application_server.http.enable_connect_protocol {
                http2_builder.enable_connect_protocol();
            };
            match environment_configuration.application_server.http.keepalive {
                Some(ref keepalive) => http2_builder
                    .keep_alive_interval(Some(Duration::from_secs(keepalive.interval_duration)))
                    .keep_alive_timeout(Duration::from_secs(keepalive.timeout_duration)),
                None => http2_builder.keep_alive_interval(None),
            };
            match environment_configuration.application_server.http.maximum_pending_accept_reset_streams {
                Some(maximum_pending_accept_reset_streams) => http2_builder.max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams)),
                None => http2_builder.max_pending_accept_reset_streams(None),
            };

            if let Some(ref _tls) = environment_configuration.application_server.http.tls {
                todo!("// TODO ssl_protocolsTLSv1 TLSv1.1 TLSv1.2 TLSv1.3;  ssl_ciphers HIGH:!aNULL:!MD5;")
            }
            let tcp_listener = TcpListener::bind(
                &environment_configuration.application_server.tcp.socket_address,
            ).await
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let serving_connection_registry_future = async move {
                '_a: loop {
                    let tcp_stream = tcp_listener.accept().await.into_runtime(     // TODO TODO TODO TODO TODO что вот здесь делаем?
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?.0;
                    let cloned_ = cloned.clone();
                    let serving_connection_future = http2_builder.serve_connection(
                        TokioIo::new(tcp_stream),
                        service_fn(
                            move |request: Request| -> _ {
                                let cloned__ = cloned_.clone();
                                return async move {
                                    let response = Self::resolve(
                                        request,
                                        environment_configuration,
                                        cloned__,
                                    )
                                    .await;
                                    return Ok::<_, Void>(response);
                                };
                            }
                        ),
                    );
                    Spawner::<TokioNonBlockingTask>::spawn_into_background(
                        async move {
                            CONNECTION_QUANTITY.fetch_add(1, Ordering::Relaxed);
                            let result = serving_connection_future.await.into_runtime(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            );
                            CONNECTION_QUANTITY.fetch_sub(1, Ordering::Relaxed);
                            return result;
                        }
                    );
                }
                return Ok::<_, AggregateError>(());
            };
            let serving_connection_registry_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(serving_connection_registry_future);
            let graceful_shutdown_signal_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(graceful_shutdown_signal_future);
            tokio::select! {
                biased;
                _ = graceful_shutdown_signal_join_handle => {
                    ()
                },
                _ = serving_connection_registry_join_handle => {
                    return Err(
                        AggregateError::new_logic_(
                            Common::UnreachableState,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                },
            }
            'a: loop {
                if CONNECTION_QUANTITY.load(Ordering::Relaxed) != 0 {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue 'a;
                } else {
                    break 'a;
                }
            }
            return Ok(());
        };
    }
    fn create_router() -> Result<Router<ActionRoute_>, AggregateError> {
        let mut router = Router::new();
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.check_nickname_for_existing,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting,
                },
            )
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
            .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
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
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
        }
        return Ok(router);
    }
    fn resolve<'a, T>(
        request: Request,
        environment_configuration: &'a EnvironmentConfiguration,
        cloned: Arc<Cloned<T>>,
    ) -> impl Future<Output = Response> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            let (parts, mut incoming) = request.into_parts();
            let mut action_inner = ActionInner {
                incoming: &mut incoming,
                parts: &parts,
            };
            let r#match = match cloned.router.at(parts.uri.path()) {
                Ok(r#match_) => r#match_,
                Err(_) => {
                    return Action::<RouteNotFound>::run(&mut action_inner);
                }
            };
            let action_processor_inner = ActionProcessorInner {
                environment_configuration,
                database_1_postgresql_connection_pool: &cloned.database_1_postgresql_connection_pool,
                database_2_postgresql_connection_pool: &cloned.database_2_postgresql_connection_pool,
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
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&ApplicationUser__Authorization_::CheckEmailForExisting, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___CheckEmailForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::RegisterByFirstStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___RegisterByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::RegisterBySecondStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___RegisterBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::RegisterByLastStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___RegisterByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::SendEmailForRegister, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___SendEmailForRegister>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::AuthorizeByFirstStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___AuthorizeByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::AuthorizeByLastStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___AuthorizeByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::SendEmailForAuthorize, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___SendEmailForAuthorize>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::ResetPasswordByFirstStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___ResetPasswordByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::ResetPasswordBySecondStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___ResetPasswordBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::ResetPasswordByLastStep, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___ResetPasswordByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::SendEmailForResetPassword, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___SendEmailForResetPassword>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::RefreshAccessToken, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___RefreshAccessToken>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___DeauthorizeFromOneDevice>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices, &Method::POST) => {
                            return Action::<ApplicationUser__Authorization___DeauthorizeFromAllDevices>::run(
                                &mut action_inner,
                                &action_processor_inner,
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
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&ApplicationUser__Authorization_::CheckEmailForExisting_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___CheckEmailForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::RegisterByFirstStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___RegisterByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::RegisterBySecondStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___RegisterBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::RegisterByLastStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___RegisterByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::SendEmailForRegister_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___SendEmailForRegister>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::AuthorizeByFirstStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___AuthorizeByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::AuthorizeByLastStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___AuthorizeByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::SendEmailForAuthorize_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___SendEmailForAuthorize>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::ResetPasswordByFirstStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___ResetPasswordByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::ResetPasswordBySecondStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___ResetPasswordBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::ResetPasswordByLastStep_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___ResetPasswordByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::SendEmailForResetPassword_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___SendEmailForResetPassword>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::RefreshAccessToken_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___RefreshAccessToken>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::DeauthorizeFromOneDevice_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___DeauthorizeFromOneDevice>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ApplicationUser__Authorization_::DeauthorizeFromAllDevices_, &Method::POST) => {
                                        return Action::<ApplicationUser__Authorization___DeauthorizeFromAllDevices>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
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
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel__Base_::GetManyByNameInSubscriptions, &Method::POST) => {
                            return Action::<Channel__Base___GetManyByNameInSubscriptions>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel__Base_::GetManyBySubscription, &Method::POST) => {
                            return Action::<Channel__Base___GetManyBySubscription>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel__Base_::GetManyPublicByName, &Method::POST) => {
                            return Action::<Channel__Base___GetManyPublicByName>::run(
                                &mut action_inner,
                                &action_processor_inner,
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
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel__Base_::GetManyByNameInSubscriptions_, &Method::POST) => {
                                        return Action::<Channel__Base___GetManyByNameInSubscriptions>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel__Base_::GetManyBySubscription_, &Method::POST) => {
                                        return Action::<Channel__Base___GetManyBySubscription>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel__Base_::GetManyPublicByName_, &Method::POST) => {
                                        return Action::<Channel__Base___GetManyPublicByName>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
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
                                &mut action_inner,
                                &action_processor_inner,
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
                                            &mut action_inner,
                                            &action_processor_inner,
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
            return Action::<RouteNotFound>::run(&mut action_inner);
        };
    }
    fn create_signal(signal_kind: SignalKind) -> Result<impl Future<Output = ()> + Send, AggregateError> {
        let mut signal = tokio::signal::unix::signal(signal_kind).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let signal_ = async move {
            signal.recv().await;
            return ();
        };
        return Ok(signal_);
    }
}
struct Cloned<T>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    router: Router<ActionRoute_>,
    database_1_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
}