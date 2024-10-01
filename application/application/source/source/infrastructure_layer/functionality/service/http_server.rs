use crate::{
    application_layer::functionality::action_processor::{
        application_user___authorization::{
            authorize_by_first_step::ApplicationUser__Authorization___AuthorizeByFirstStep,
            authorize_by_last_step::ApplicationUser__Authorization___AuthorizeByLastStep,
            check_email_for_existing::ApplicationUser__Authorization___CheckEmailForExisting,
            check_nickname_for_existing::ApplicationUser__Authorization___CheckNicknameForExisting,
            deauthorize_from_all_devices::ApplicationUser__Authorization___DeauthorizeFromAllDevices,
            deauthorize_from_one_device::ApplicationUser__Authorization___DeauthorizeFromOneDevice,
            refresh_access_token::ApplicationUser__Authorization___RefreshAccessToken,
            register_by_first_step::ApplicationUser__Authorization___RegisterByFirstStep,
            register_by_last_step::ApplicationUser__Authorization___RegisterByLastStep,
            register_by_second_step::ApplicationUser__Authorization___RegisterBySecondStep,
            reset_password_by_first_step::ApplicationUser__Authorization___ResetPasswordByFirstStep,
            reset_password_by_last_step::ApplicationUser__Authorization___ResetPasswordByLastStep,
            reset_password_by_second_step::ApplicationUser__Authorization___ResetPasswordBySecondStep,
            send_email_for_authorize::ApplicationUser__Authorization___SendEmailForAuthorize,
            send_email_for_register::ApplicationUser__Authorization___SendEmailForRegister,
            send_email_for_reset_password::ApplicationUser__Authorization___SendEmailForResetPassword,
        },
        channel___base::{
            get_many_by_name_in_subscriptions::Channel__Base___GetManyByNameInSubscriptions,
            get_many_by_subscription::Channel__Base___GetManyBySubscription,
            get_many_public_by_name::Channel__Base___GetManyPublicByName,
            get_one_by_id::Channel__Base___GetOneById,
        },
        channel_subscription___base::create::ChannelSubscription__Base___Create,
        Inner as ActionProcessorInner,
    },
    infrastructure_layer::{
        data::{
            control_type::{
                Request,
                Response,
            },
            environment_configuration::environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::{
            creator::Creator,
            logger::Logger,
            spawner::{
                tokio_non_blocking_task::TokioNonBlockingTask,
                Spawner,
            },
        },
    },
    presentation_layer::functionality::action::{
        route_not_found::RouteNotFound,
        Action,
        Inner as ActionInner,
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
#[cfg(feature = "manual_testing")]
use core::net::SocketAddr;
#[cfg(feature = "manual_testing")]
use hyper::server::conn::http1::Builder as Http1Builder;
use hyper::{
    server::conn::http2::Builder as Http2Builder,
    Method,
};
use hyper_util::rt::{
    tokio::TokioExecutor,
    TokioIo,
};
use std::{
    error::Error,
    future::Future,
    sync::{
        atomic::{
            AtomicU64,
            Ordering,
        },
        Arc,
    },
    time::Duration,
};
use matchit::Router;
use tokio::{
    net::TcpListener,
    signal::unix::SignalKind,
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    NoTls,
    Socket,
};
use void::Void;
#[cfg(feature = "manual_testing")]
use const_format::concatcp;
pub const ACTION_ROUTE: ActionRoute = ActionRoute {
    application_user___authorization: ApplicationUser__Authorization {
        check_nickname_for_existing: ApplicationUser__Authorization::CHECK_NICKNAME_FOR_EXISTING,
        check_email_for_existing: ApplicationUser__Authorization::CHECK_EMAIL_FOR_EXISTING,
        regisgter_by_first_step: ApplicationUser__Authorization::REGISTER_BY_FIRST_STEP,
        regisgter_by_second_step: ApplicationUser__Authorization::REGISTER_BY_SECOND_STEP,
        regisgter_by_last_step: ApplicationUser__Authorization::REGISTER_BY_LAST_STEP,
        send_email_for_register: ApplicationUser__Authorization::SEND_EMAIL_FOR_REGISTER,
        authorize_by_first_step: ApplicationUser__Authorization::AUTHORIZE_BY_FIRST_STEP,
        authorize_by_last_step: ApplicationUser__Authorization::AUTHORIZE_BY_LAST_STEP,
        send_email_for_authorize: ApplicationUser__Authorization::SEND_EMAIL_FOR_AUTHORIZE,
        reset_password_by_first_step: ApplicationUser__Authorization::RESET_PASSWORD_BY_FIRST_STEP,
        reset_password_by_second_step: ApplicationUser__Authorization::RESET_PASSWORD_BY_SECOND_STEP,
        reset_password_by_last_step: ApplicationUser__Authorization::RESET_PASSWORD_BY_LAST_STEP,
        send_email_for_reset_password: ApplicationUser__Authorization::SEND_EMAIL_FOR_RESET_PASSWORD,
        refresh_access_token: ApplicationUser__Authorization::REFRESH_ACCESS_TOKEN,
        deauthorize_from_one_device: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ONE_DEVICE,
        deauthorize_from_all_devices: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ALL_DEVICES,
        #[cfg(feature = "manual_testing")]
        check_nickname_for_existing_: ApplicationUser__Authorization::CHECK_NICKNAME_FOR_EXISTING_,
        #[cfg(feature = "manual_testing")]
        check_email_for_existing_: ApplicationUser__Authorization::CHECK_EMAIL_FOR_EXISTING_,
        #[cfg(feature = "manual_testing")]
        regisgter_by_first_step_: ApplicationUser__Authorization::REGISTER_BY_FIRST_STEP_,
        #[cfg(feature = "manual_testing")]
        regisgter_by_second_step_: ApplicationUser__Authorization::REGISTER_BY_SECOND_STEP_,
        #[cfg(feature = "manual_testing")]
        regisgter_by_last_step_: ApplicationUser__Authorization::REGISTER_BY_LAST_STEP_,
        #[cfg(feature = "manual_testing")]
        send_email_for_register_: ApplicationUser__Authorization::SEND_EMAIL_FOR_REGISTER_,
        #[cfg(feature = "manual_testing")]
        authorize_by_first_step_: ApplicationUser__Authorization::AUTHORIZE_BY_FIRST_STEP_,
        #[cfg(feature = "manual_testing")]
        authorize_by_last_step_: ApplicationUser__Authorization::AUTHORIZE_BY_LAST_STEP_,
        #[cfg(feature = "manual_testing")]
        send_email_for_authorize_: ApplicationUser__Authorization::SEND_EMAIL_FOR_AUTHORIZE_,
        #[cfg(feature = "manual_testing")]
        reset_password_by_first_step_: ApplicationUser__Authorization::RESET_PASSWORD_BY_FIRST_STEP_,
        #[cfg(feature = "manual_testing")]
        reset_password_by_second_step_: ApplicationUser__Authorization::RESET_PASSWORD_BY_SECOND_STEP_,
        #[cfg(feature = "manual_testing")]
        reset_password_by_last_step_: ApplicationUser__Authorization::RESET_PASSWORD_BY_LAST_STEP_,
        #[cfg(feature = "manual_testing")]
        send_email_for_reset_password_: ApplicationUser__Authorization::SEND_EMAIL_FOR_RESET_PASSWORD_,
        #[cfg(feature = "manual_testing")]
        refresh_access_token_: ApplicationUser__Authorization::REFRESH_ACCESS_TOKEN_,
        #[cfg(feature = "manual_testing")]
        deauthorize_from_one_device_: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ONE_DEVICE_,
        #[cfg(feature = "manual_testing")]
        deauthorize_from_all_devices_: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ALL_DEVICES_,
    },
    channel___base: Channel__Base {
        get_one_by_id: Channel__Base::GET_ONE_BY_ID,
        get_many_by_name_in_subscription: Channel__Base::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        get_many_by_subscription: Channel__Base::GET_MANY_BY_SUBSCRIPTION,
        get_many_public_by_name: Channel__Base::GET_MANY_PUBLIC_BY_NAME,
        create: Channel__Base::CREATE,
        check_name_for_existing: Channel__Base::CHECK_NAME_FOR_EXISTING,
        check_linked_name_for_existing: Channel__Base::CHECK_LINKED_NAME_FOR_EXISTING,
        #[cfg(feature = "manual_testing")]
        get_one_by_id_: Channel__Base::GET_ONE_BY_ID_,
        #[cfg(feature = "manual_testing")]
        get_many_by_name_in_subscription_: Channel__Base::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_,
        #[cfg(feature = "manual_testing")]
        get_many_by_subscription_: Channel__Base::GET_MANY_BY_SUBSCRIPTION_,
        #[cfg(feature = "manual_testing")]
        get_many_public_by_name_: Channel__Base::GET_MANY_PUBLIC_BY_NAME_,
        #[cfg(feature = "manual_testing")]
        create_: Channel__Base::CREATE_,
        #[cfg(feature = "manual_testing")]
        check_name_for_existing_: Channel__Base::CHECK_NAME_FOR_EXISTING_,
        #[cfg(feature = "manual_testing")]
        check_linked_name_for_existing_: Channel__Base::CHECK_LINKED_NAME_FOR_EXISTING_,
    },
    channel_subscription___base: ChannelSubscription__Base {
        create: ChannelSubscription__Base::CREATE,
        #[cfg(feature = "manual_testing")]
        create_: ChannelSubscription__Base::CREATE_,
    },
};
static CONNECTION_QUANTITY: AtomicU64 = AtomicU64::new(0);
pub struct HttpServer;
impl HttpServer {
    pub fn run(environment_configuration: &'static EnvironmentConfiguration) -> impl Future<Output = Result<(), AggregateError>> + Send {
        return async move {
            #[cfg(feature = "manual_testing")]
            let http1_socket_address = {
                let mut http1_port_number = environment_configuration.application_server.tcp.socket_address.port();
                if http1_port_number >= u16::MIN && http1_port_number < u16::MAX {
                    http1_port_number += 1;
                } else {
                    http1_port_number -= 1;
                };
                SocketAddr::new(
                    environment_configuration.application_server.tcp.socket_address.ip(),
                    http1_port_number,
                )
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
                return ();
            };
            let mut graceful_shutdown_signal_future_ = std::pin::pin!(graceful_shutdown_signal_future);
            let cloned = Arc::new(
                Cloned {
                    router: Self::create_router()?,
                    database_1_postgresql_connection_pool: Creator::<Pool<PostgresConnectionManager<NoTls>>>::create(
                        environment_configuration.resource.postgresql.database_1_url.as_str(),
                    )
                    .await?,
                    database_2_postgresql_connection_pool: Creator::<Pool<PostgresConnectionManager<NoTls>>>::create(
                        environment_configuration.resource.postgresql.database_2_url.as_str(),
                    )
                    .await?,
                },
            );
            'a: loop {
                let cloned_ = cloned.clone();
                let mut graceful_shutdown_signal_future__ = graceful_shutdown_signal_future_.as_mut();
                #[cfg(feature = "manual_testing")]
                let http1_tcp_listener = TcpListener::bind(&http1_socket_address).await.into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                let http2_tcp_listener = TcpListener::bind(&environment_configuration.application_server.tcp.socket_address).await.into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                let future = async move {
                    #[cfg(feature = "manual_testing")]
                    let http1_builder = Http1Builder::new();
                    let mut http2_builder = Http2Builder::new(TokioExecutor::new()).max_local_error_reset_streams(Option::Some(128));
                    http2_builder
                        .auto_date_header(false)
                        .max_header_list_size(environment_configuration.application_server.http.maximum_header_list_size)
                        .adaptive_window(environment_configuration.application_server.http.adaptive_window)
                        .initial_connection_window_size(Option::Some(environment_configuration.application_server.http.connection_window_size))
                        .initial_stream_window_size(Option::Some(environment_configuration.application_server.http.stream_window_size))
                        .max_concurrent_streams(Option::None)
                        .max_frame_size(Option::Some(environment_configuration.application_server.http.maximum_frame_size))
                        .max_send_buf_size(environment_configuration.application_server.http.maximum_sending_buffer_size as usize);
                    if environment_configuration.application_server.http.enable_connect_protocol {
                        http2_builder.enable_connect_protocol();
                    };
                    match environment_configuration.application_server.http.keepalive {
                        Option::Some(ref keepalive) => {
                            http2_builder
                                .keep_alive_interval(Option::Some(Duration::from_secs(keepalive.interval_duration)))
                                .keep_alive_timeout(Duration::from_secs(keepalive.timeout_duration))
                        }
                        Option::None => http2_builder.keep_alive_interval(Option::None),
                    };
                    match environment_configuration.application_server.http.maximum_pending_accept_reset_streams {
                        Option::Some(maximum_pending_accept_reset_streams) => http2_builder.max_pending_accept_reset_streams(Option::Some(maximum_pending_accept_reset_streams)),
                        Option::None => http2_builder.max_pending_accept_reset_streams(Option::None),
                    };
                    if let Option::Some(ref _tls) = environment_configuration.application_server.http.tls {
                        todo!("// TODO ssl_protocolsTLSv1 TLSv1.1 TLSv1.2 TLSv1.3;  ssl_ciphers HIGH:!aNULL:!MD5;")
                    }
                    'b: loop {
                        #[cfg(not(feature = "manual_testing"))]
                        let tcp_accepting_future = http2_tcp_listener.accept();
                        #[cfg(feature = "manual_testing")]
                        let tcp_accepting_future = async {
                            return tokio::select! {
                                result = http1_tcp_listener.accept() => {
                                    result
                                },
                                result = http2_tcp_listener.accept() => {
                                    result
                                },
                            };
                        };
                        tokio::select! {
                            biased;
                            _ = graceful_shutdown_signal_future__.as_mut() => {
                                break 'b;
                            },
                            tcp_stream = tcp_accepting_future => {
                                let tcp_stream_ = match tcp_stream {
                                    Result::Ok((tcp_stream__, _)) => tcp_stream__,
                                    Result::Err(error) => {
                                        Spawner::<TokioNonBlockingTask>::spawn_into_background(
                                            async move {
                                                Logger::<AggregateError>::log(
                                                    &AggregateError::new_runtime(
                                                        error.into(),
                                                        Backtrace::new(
                                                            line!(),
                                                            file!(),
                                                        ),
                                                    )
                                                );
                                                return Result::Ok(());
                                            }
                                        );
                                        continue 'b;
                                    }
                                };
                                let cloned__ = cloned_.clone();
                                let service_fn = hyper::service::service_fn(
                                    move |request: Request| -> _ {
                                        let cloned___ = cloned__.clone();
                                        return async move {
                                            let response = Self::process_request(
                                                request,
                                                environment_configuration,
                                                cloned___,
                                            )
                                            .await;
                                            return Result::<_, Void>::Ok(response);
                                        };
                                    },
                                );
                                #[cfg(feature = "manual_testing")]
                                {
                                    let socket_address_port = tcp_stream_.local_addr().into_logic(
                                        Backtrace::new(
                                            line!(),
                                            file!(),
                                        ),
                                    )?;
                                    if http1_socket_address.port() == socket_address_port.port() {
                                        Self::spawn_connection_serving(
                                            http1_builder.serve_connection(
                                                TokioIo::new(tcp_stream_),
                                                service_fn,
                                            ),
                                        );
                                    } else {
                                        Self::spawn_connection_serving(
                                            http2_builder.serve_connection(
                                                TokioIo::new(tcp_stream_),
                                                service_fn,
                                            ),
                                        );
                                    };
                                }
                                #[cfg(not(feature = "manual_testing"))]
                                Self::spawn_connection_serving(
                                    http2_builder.serve_connection(
                                        TokioIo::new(tcp_stream_),
                                        service_fn,
                                    ),
                                );
                                continue 'b;
                            },
                        }
                    }
                    let completion_by_connection_quantity_future = async {
                        'b: loop {
                            if CONNECTION_QUANTITY.load(Ordering::Relaxed) != 0 {
                                tokio::time::sleep(Duration::from_millis(100)).await;
                                continue 'b;
                            } else {
                                break 'b;
                            }
                        }
                        return ();
                    };
                    let completion_by_timer_future = async {
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        return ();
                    };
                    let completion_by_connection_quantity_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(completion_by_connection_quantity_future);
                    let completion_by_timer_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(completion_by_timer_future);
                    tokio::select! {
                        biased;
                        _ = completion_by_connection_quantity_join_handle => {
                            ()
                        },
                        _ = completion_by_timer_join_handle => {
                            ()
                        },
                    }
                    return Result::<_, AggregateError>::Ok(());
                };
                if let Result::Err(aggregate_error) = future.await {
                    Logger::<AggregateError>::log(&aggregate_error);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue 'a;
                }
                break 'a;
            }
            return Result::Ok(());
        };
    }
    fn create_router() -> Result<Router<ActionRoute_>, AggregateError> {
        let mut router = Router::<ActionRoute_>::new();
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
                ACTION_ROUTE.channel___base.get_many_public_by_name,
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
                ACTION_ROUTE.channel___base.create,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::Create,
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
                ACTION_ROUTE.channel___base.check_name_for_existing,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::CheckNameForExisting,
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
                ACTION_ROUTE.channel___base.check_linked_name_for_existing,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::CheckLinkedNameForExisting,
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
                    ACTION_ROUTE.channel___base.get_many_public_by_name_,
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
                    ACTION_ROUTE.channel___base.create_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::Create_,
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
                    ACTION_ROUTE.channel___base.check_name_for_existing_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::CheckNameForExisting_,
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
                    ACTION_ROUTE.channel___base.check_linked_name_for_existing_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::CheckLinkedNameForExisting_,
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
        return Result::Ok(router);
    }
    fn spawn_connection_serving<T, E>(serving_connection_future: T) -> ()
    where
        T: Future<Output = Result<(), E>> + Send + 'static,
        E: Error + Send + Sync + 'static,
    {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                const STEP: u64 = 1;
                CONNECTION_QUANTITY.fetch_add(
                    STEP,
                    Ordering::Relaxed,
                ); // TODO TODO TODO TODO TODO TODO TODO TODO TODO TODO TODO TODO TODO TODO нужно ли catch_unwind.
                let result = serving_connection_future.await.into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                );
                CONNECTION_QUANTITY.fetch_sub(
                    STEP,
                    Ordering::Relaxed,
                );
                return result;
            },
        );
        return ();
    }
    fn process_request<T>(
        request: Request,
        environment_configuration: &'static EnvironmentConfiguration,
        cloned: Arc<Cloned<T>>
    ) -> impl Future<Output = Response> + Send
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
                Result::Ok(r#match_) => r#match_,
                Result::Err(_) => {
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
        let signal_future = async move {
            signal.recv().await;
            return ();
        };
        return Result::Ok(signal_future);
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
    database_1_postgresql_connection_pool: Pool<PostgresConnectionManager<T>>,
    database_2_postgresql_connection_pool: Pool<PostgresConnectionManager<T>>,
}
pub struct ActionRoute {
    pub application_user___authorization: ApplicationUser__Authorization,
    pub channel___base: Channel__Base,
    pub channel_subscription___base: ChannelSubscription__Base,
}
#[cfg(feature = "manual_testing")]
impl ActionRoute {
    const CONCATENATING_PART: &'static str = "_";
}
pub struct ApplicationUser__Authorization {
    pub check_nickname_for_existing: &'static str,
    pub check_email_for_existing: &'static str,
    pub regisgter_by_first_step: &'static str,
    pub regisgter_by_second_step: &'static str,
    pub regisgter_by_last_step: &'static str,
    pub send_email_for_register: &'static str,
    pub authorize_by_first_step: &'static str,
    pub authorize_by_last_step: &'static str,
    pub send_email_for_authorize: &'static str,
    pub reset_password_by_first_step: &'static str,
    pub reset_password_by_second_step: &'static str,
    pub reset_password_by_last_step: &'static str,
    pub send_email_for_reset_password: &'static str,
    pub refresh_access_token: &'static str,
    pub deauthorize_from_one_device: &'static str,
    pub deauthorize_from_all_devices: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_nickname_for_existing_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_email_for_existing_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub regisgter_by_first_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub regisgter_by_second_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub regisgter_by_last_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub send_email_for_register_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub authorize_by_first_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub authorize_by_last_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub send_email_for_authorize_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub reset_password_by_first_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub reset_password_by_second_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub reset_password_by_last_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub send_email_for_reset_password_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub refresh_access_token_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub deauthorize_from_one_device_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub deauthorize_from_all_devices_: &'static str,
}
impl ApplicationUser__Authorization {
    const AUTHORIZE_BY_FIRST_STEP: &'static str = "/1/7";
    const AUTHORIZE_BY_LAST_STEP: &'static str = "/1/8";
    const CHECK_EMAIL_FOR_EXISTING: &'static str = "/1/2";
    const CHECK_NICKNAME_FOR_EXISTING: &'static str = "/1/1";
    const DEAUTHORIZE_FROM_ALL_DEVICES: &'static str = "/1/16";
    const DEAUTHORIZE_FROM_ONE_DEVICE: &'static str = "/1/15";
    const REFRESH_ACCESS_TOKEN: &'static str = "/1/14";
    const REGISTER_BY_FIRST_STEP: &'static str = "/1/3";
    const REGISTER_BY_LAST_STEP: &'static str = "/1/5";
    const REGISTER_BY_SECOND_STEP: &'static str = "/1/4";
    const RESET_PASSWORD_BY_FIRST_STEP: &'static str = "/1/10";
    const RESET_PASSWORD_BY_LAST_STEP: &'static str = "/1/12";
    const RESET_PASSWORD_BY_SECOND_STEP: &'static str = "/1/11";
    const SEND_EMAIL_FOR_AUTHORIZE: &'static str = "/1/9";
    const SEND_EMAIL_FOR_REGISTER: &'static str = "/1/6";
    const SEND_EMAIL_FOR_RESET_PASSWORD: &'static str = "/1/13";
}
#[cfg(feature = "manual_testing")]
impl ApplicationUser__Authorization {
    const AUTHORIZE_BY_FIRST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::AUTHORIZE_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const AUTHORIZE_BY_LAST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::AUTHORIZE_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_EMAIL_FOR_EXISTING_: &'static str = concatcp!(
        ApplicationUser__Authorization::CHECK_EMAIL_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_NICKNAME_FOR_EXISTING_: &'static str = concatcp!(
        ApplicationUser__Authorization::CHECK_NICKNAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    const DEAUTHORIZE_FROM_ALL_DEVICES_: &'static str = concatcp!(
        ApplicationUser__Authorization::DEAUTHORIZE_FROM_ALL_DEVICES,
        ActionRoute::CONCATENATING_PART
    );
    const DEAUTHORIZE_FROM_ONE_DEVICE_: &'static str = concatcp!(
        ApplicationUser__Authorization::DEAUTHORIZE_FROM_ONE_DEVICE,
        ActionRoute::CONCATENATING_PART
    );
    const REFRESH_ACCESS_TOKEN_: &'static str = concatcp!(
        ApplicationUser__Authorization::REFRESH_ACCESS_TOKEN,
        ActionRoute::CONCATENATING_PART
    );
    const REGISTER_BY_FIRST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::REGISTER_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const REGISTER_BY_LAST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::REGISTER_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const REGISTER_BY_SECOND_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::REGISTER_BY_SECOND_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const RESET_PASSWORD_BY_FIRST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::RESET_PASSWORD_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const RESET_PASSWORD_BY_LAST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::RESET_PASSWORD_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const RESET_PASSWORD_BY_SECOND_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::RESET_PASSWORD_BY_SECOND_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const SEND_EMAIL_FOR_AUTHORIZE_: &'static str = concatcp!(
        ApplicationUser__Authorization::SEND_EMAIL_FOR_AUTHORIZE,
        ActionRoute::CONCATENATING_PART
    );
    const SEND_EMAIL_FOR_REGISTER_: &'static str = concatcp!(
        ApplicationUser__Authorization::SEND_EMAIL_FOR_REGISTER,
        ActionRoute::CONCATENATING_PART
    );
    const SEND_EMAIL_FOR_RESET_PASSWORD_: &'static str = concatcp!(
        ApplicationUser__Authorization::SEND_EMAIL_FOR_RESET_PASSWORD,
        ActionRoute::CONCATENATING_PART
    );
}
pub struct Channel__Base {
    pub get_one_by_id: &'static str,
    pub get_many_by_name_in_subscription: &'static str,
    pub get_many_by_subscription: &'static str,
    pub get_many_public_by_name: &'static str,
    pub create: &'static str,
    pub check_name_for_existing: &'static str,
    pub check_linked_name_for_existing: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_one_by_id_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_many_by_name_in_subscription_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_many_by_subscription_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_many_public_by_name_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub create_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_name_for_existing_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_linked_name_for_existing_: &'static str,
}
impl Channel__Base {
    const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS: &'static str = "/1/18";
    const GET_MANY_BY_SUBSCRIPTION: &'static str = "/1/19";
    const GET_MANY_PUBLIC_BY_NAME: &'static str = "/1/20";
    const GET_ONE_BY_ID: &'static str = "/1/17";
    const CREATE: &'static str = "/1/22";
    const CHECK_NAME_FOR_EXISTING: &'static str = "/1/23";
    const CHECK_LINKED_NAME_FOR_EXISTING: &'static str = "/1/24";
}
#[cfg(feature = "manual_testing")]
impl Channel__Base {
    const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_: &'static str = concatcp!(
        Channel__Base::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        ActionRoute::CONCATENATING_PART
    );
    const GET_MANY_BY_SUBSCRIPTION_: &'static str = concatcp!(
        Channel__Base::GET_MANY_BY_SUBSCRIPTION,
        ActionRoute::CONCATENATING_PART
    );
    const GET_MANY_PUBLIC_BY_NAME_: &'static str = concatcp!(
        Channel__Base::GET_MANY_PUBLIC_BY_NAME,
        ActionRoute::CONCATENATING_PART
    );
    const GET_ONE_BY_ID_: &'static str = concatcp!(
        Channel__Base::GET_ONE_BY_ID,
        ActionRoute::CONCATENATING_PART
    );
    const CREATE_: &'static str = concatcp!(
        Channel__Base::CREATE,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_NAME_FOR_EXISTING_: &'static str = concatcp!(
        Channel__Base::CHECK_NAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_LINKED_NAME_FOR_EXISTING_: &'static str = concatcp!(
        Channel__Base::CHECK_LINKED_NAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
}
pub struct ChannelSubscription__Base {
    pub create: &'static str,
    #[cfg(feature = "manual_testing")]
    pub create_: &'static str,
}
impl ChannelSubscription__Base {
    const CREATE: &'static str = "/1/21";
}
#[cfg(feature = "manual_testing")]
impl ChannelSubscription__Base {
    const CREATE_: &'static str = concatcp!(
        ChannelSubscription__Base::CREATE,
        ActionRoute::CONCATENATING_PART
    );
}
pub enum ActionRoute_ {
    ApplicationUser__Authorization {
        application_user___authorization: ApplicationUser__Authorization_,
    },
    Channel__Base {
        channel___base: Channel__Base_,
    },
    ChannelSubscription__Base {
        channel_subscription___base: ChannelSubscription__Base_,
    },
}
pub enum ApplicationUser__Authorization_ {
    CheckNicknameForExisting,
    CheckEmailForExisting,
    RegisterByFirstStep,
    RegisterBySecondStep,
    RegisterByLastStep,
    SendEmailForRegister,
    AuthorizeByFirstStep,
    AuthorizeByLastStep,
    SendEmailForAuthorize,
    ResetPasswordByFirstStep,
    ResetPasswordBySecondStep,
    ResetPasswordByLastStep,
    SendEmailForResetPassword,
    RefreshAccessToken,
    DeauthorizeFromOneDevice,
    DeauthorizeFromAllDevices,
    #[cfg(feature = "manual_testing")]
    CheckNicknameForExisting_,
    #[cfg(feature = "manual_testing")]
    CheckEmailForExisting_,
    #[cfg(feature = "manual_testing")]
    RegisterByFirstStep_,
    #[cfg(feature = "manual_testing")]
    RegisterBySecondStep_,
    #[cfg(feature = "manual_testing")]
    RegisterByLastStep_,
    #[cfg(feature = "manual_testing")]
    SendEmailForRegister_,
    #[cfg(feature = "manual_testing")]
    AuthorizeByFirstStep_,
    #[cfg(feature = "manual_testing")]
    AuthorizeByLastStep_,
    #[cfg(feature = "manual_testing")]
    SendEmailForAuthorize_,
    #[cfg(feature = "manual_testing")]
    ResetPasswordByFirstStep_,
    #[cfg(feature = "manual_testing")]
    ResetPasswordBySecondStep_,
    #[cfg(feature = "manual_testing")]
    ResetPasswordByLastStep_,
    #[cfg(feature = "manual_testing")]
    SendEmailForResetPassword_,
    #[cfg(feature = "manual_testing")]
    RefreshAccessToken_,
    #[cfg(feature = "manual_testing")]
    DeauthorizeFromOneDevice_,
    #[cfg(feature = "manual_testing")]
    DeauthorizeFromAllDevices_,
}
pub enum Channel__Base_ {
    GetOneById,
    GetManyByNameInSubscriptions,
    GetManyBySubscription,
    GetManyPublicByName,
    Create,
    CheckNameForExisting,
    CheckLinkedNameForExisting,
    #[cfg(feature = "manual_testing")]
    GetOneById_,
    #[cfg(feature = "manual_testing")]
    GetManyByNameInSubscriptions_,
    #[cfg(feature = "manual_testing")]
    GetManyBySubscription_,
    #[cfg(feature = "manual_testing")]
    GetManyPublicByName_,
    #[cfg(feature = "manual_testing")]
    Create_,
    #[cfg(feature = "manual_testing")]
    CheckNameForExisting_,
    #[cfg(feature = "manual_testing")]
    CheckLinkedNameForExisting_,
}
pub enum ChannelSubscription__Base_ {
    Create,
    #[cfg(feature = "manual_testing")]
    Create_,
}
