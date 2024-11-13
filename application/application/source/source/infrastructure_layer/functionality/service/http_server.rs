#[cfg(not(feature = "postgresql_connection_with_tls"))]
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::{
    application_layer::functionality::action_processor::{
        ChannelSubscription_Create,
        Channel_CheckLinkedNameForExisting,
        Channel_CheckNameForExisting,
        Channel_Create,
        Channel_GetManyByNameInSubscriptions,
        Channel_GetManyBySubscription,
        Channel_GetManyPublicByName,
        Channel_GetOneById,
        Inner as ActionProcessorInner,
        UserAuthorization_AuthorizeByFirstStep,
        UserAuthorization_AuthorizeByLastStep,
        UserAuthorization_CheckEmailForExisting,
        UserAuthorization_CheckNicknameForExisting,
        UserAuthorization_DeauthorizeFromAllDevices,
        UserAuthorization_DeauthorizeFromOneDevice,
        UserAuthorization_RefreshAccessToken,
        UserAuthorization_RegisterByFirstStep,
        UserAuthorization_RegisterByLastStep,
        UserAuthorization_RegisterBySecondStep,
        UserAuthorization_ResetPasswordByFirstStep,
        UserAuthorization_ResetPasswordByLastStep,
        UserAuthorization_ResetPasswordBySecondStep,
        UserAuthorization_SendEmailForAuthorize,
        UserAuthorization_SendEmailForRegister,
        UserAuthorization_SendEmailForResetPassword,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            control_type::{
                Request,
                Response,
            },
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::{
            creator::PostgresqlConnectionPool,
            logger::Logger,
            spawner::{
                Spawner,
                TokioNonBlockingTask,
            },
        },
    },
    presentation_layer::functionality::action::{
        Action,
        Inner as ActionInner,
        RouteNotFound,
    },
};
#[cfg(feature = "port_for_manual_test")]
use core::net::SocketAddr;
use dedicated_crate::void::Void;
#[cfg(feature = "port_for_manual_test")]
use hyper::server::conn::http1::Builder as Http1Builder;
use hyper::{
    server::conn::http2::Builder as Http2Builder,
    Method,
};
use hyper_util::rt::{
    tokio::TokioExecutor,
    TokioIo,
};
use matchit::Router;
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
use tokio::{
    net::TcpListener,
    signal::unix::SignalKind,
};
#[cfg(not(feature = "postgresql_connection_with_tls"))]
use tokio_postgres::NoTls;
static CONNECTION_QUANTITY: AtomicU64 = AtomicU64::new(0);
pub struct HttpServer;
impl HttpServer {
    pub fn run(environment_configuration: &'static EnvironmentConfiguration) -> impl Future<Output = Result<(), AggregateError>> + Send {
        return async move {
            #[cfg(feature = "port_for_manual_test")]
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
                    _ = signal_interrupt_future => {},
                    _ = signal_terminate_future => {},
                }
                return ();
            };
            let mut graceful_shutdown_signal_future_ = std::pin::pin!(graceful_shutdown_signal_future);
            let database_1_postgresql_connection_pool;
            let database_2_postgresql_connection_pool;
            #[cfg(feature = "postgresql_connection_with_tls")]
            {
                todo!();
            }
            #[cfg(not(feature = "postgresql_connection_with_tls"))]
            {
                database_1_postgresql_connection_pool = Creator::<PostgresqlConnectionPool>::create(
                    &environment_configuration.resource.postgresql.database_1,
                    NoTls,
                )
                .await?;
                database_2_postgresql_connection_pool = Creator::<PostgresqlConnectionPool>::create(
                    &environment_configuration.resource.postgresql.database_2,
                    NoTls,
                )
                .await?;
            }
            let cloned = Arc::new(
                Cloned {
                    router: Self::create_router()?,
                    database_1_postgresql_connection_pool,
                    database_2_postgresql_connection_pool,
                },
            );
            'a: loop {
                let cloned_ = cloned.clone();
                let mut graceful_shutdown_signal_future__ = graceful_shutdown_signal_future_.as_mut();
                #[cfg(feature = "port_for_manual_test")]
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
                    #[cfg(feature = "port_for_manual_test")]
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
                        #[cfg(not(feature = "port_for_manual_test"))]
                        let tcp_accepting_future = http2_tcp_listener.accept();
                        #[cfg(feature = "port_for_manual_test")]
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
                                #[cfg(feature = "port_for_manual_test")]
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
                                #[cfg(not(feature = "port_for_manual_test"))]
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
                        _ = completion_by_connection_quantity_join_handle => {},
                        _ = completion_by_timer_join_handle => {},
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
    fn create_router() -> Result<Router<ActionRoute>, AggregateError> {
        let mut router = Router::<ActionRoute>::new();
        router
            .insert(
                UserAuthorization::CHECK_NICKNAME_FOR_EXISTING,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::CheckNicknameForExisting,
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
                UserAuthorization::CHECK_EMAIL_FOR_EXISTING,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::CheckEmailForExisting,
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
                UserAuthorization::REGISTER_BY_FIRST_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::RegisterByFirstStep,
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
                UserAuthorization::REGISTER_BY_SECOND_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::RegisterBySecondStep,
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
                UserAuthorization::REGISTER_BY_LAST_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::RegisterByLastStep,
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
                UserAuthorization::SEND_EMAIL_FOR_REGISTER,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::SendEmailForRegister,
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
                UserAuthorization::AUTHORIZE_BY_FIRST_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::AuthorizeByFirstStep,
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
                UserAuthorization::AUTHORIZE_BY_LAST_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::AuthorizeByLastStep,
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
                UserAuthorization::SEND_EMAIL_FOR_AUTHORIZE,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::SendEmailForAuthorize,
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
                UserAuthorization::RESET_PASSWORD_BY_FIRST_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::ResetPasswordByFirstStep,
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
                UserAuthorization::RESET_PASSWORD_BY_SECOND_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::ResetPasswordBySecondStep,
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
                UserAuthorization::RESET_PASSWORD_BY_LAST_STEP,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::ResetPasswordByLastStep,
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
                UserAuthorization::SEND_EMAIL_FOR_RESET_PASSWORD,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::SendEmailForResetPassword,
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
                UserAuthorization::REFRESH_ACCESS_TOKEN,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::RefreshAccessToken,
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
                UserAuthorization::DEAUTHORIZE_FROM_ONE_DEVICE,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::DeauthorizeFromOneDevice,
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
                UserAuthorization::DEAUTHORIZE_FROM_ALL_DEVICES,
                ActionRoute::UserAuthorization {
                    user_authorization: UserAuthorization::DeauthorizeFromAllDevices,
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
                Channel::GET_ONE_BY_ID,
                ActionRoute::Channel {
                    channel: Channel::GetOneById,
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
                Channel::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
                ActionRoute::Channel {
                    channel: Channel::GetManyByNameInSubscriptions,
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
                Channel::GET_MANY_BY_SUBSCRIPTION,
                ActionRoute::Channel {
                    channel: Channel::GetManyBySubscription,
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
                Channel::GET_MANY_PUBLIC_BY_NAME,
                ActionRoute::Channel {
                    channel: Channel::GetManyPublicByName,
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
                Channel::CREATE,
                ActionRoute::Channel {
                    channel: Channel::Create,
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
                Channel::CHECK_NAME_FOR_EXISTING,
                ActionRoute::Channel {
                    channel: Channel::CheckNameForExisting,
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
                Channel::CHECK_LINKED_NAME_FOR_EXISTING,
                ActionRoute::Channel {
                    channel: Channel::CheckLinkedNameForExisting,
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
                ChannelSubscription::CREATE,
                ActionRoute::ChannelSubscription {
                    channel_subscription: ChannelSubscription::Create,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        #[cfg(feature = "action_for_manual_test")]
        {
            router
                .insert(
                    UserAuthorization::CHECK_NICKNAME_FOR_EXISTING_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::CheckNicknameForExisting_,
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
                    UserAuthorization::CHECK_EMAIL_FOR_EXISTING_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::CheckEmailForExisting_,
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
                    UserAuthorization::REGISTER_BY_FIRST_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::RegisterByFirstStep_,
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
                    UserAuthorization::REGISTER_BY_SECOND_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::RegisterBySecondStep_,
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
                    UserAuthorization::REGISTER_BY_LAST_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::RegisterByLastStep_,
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
                    UserAuthorization::SEND_EMAIL_FOR_REGISTER_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::SendEmailForRegister_,
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
                    UserAuthorization::AUTHORIZE_BY_FIRST_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::AuthorizeByFirstStep_,
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
                    UserAuthorization::AUTHORIZE_BY_LAST_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::AuthorizeByLastStep_,
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
                    UserAuthorization::SEND_EMAIL_FOR_AUTHORIZE_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::SendEmailForAuthorize_,
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
                    UserAuthorization::RESET_PASSWORD_BY_FIRST_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::ResetPasswordByFirstStep_,
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
                    UserAuthorization::RESET_PASSWORD_BY_SECOND_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::ResetPasswordBySecondStep_,
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
                    UserAuthorization::RESET_PASSWORD_BY_LAST_STEP_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::ResetPasswordByLastStep_,
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
                    UserAuthorization::SEND_EMAIL_FOR_RESET_PASSWORD_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::SendEmailForResetPassword_,
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
                    UserAuthorization::REFRESH_ACCESS_TOKEN_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::RefreshAccessToken_,
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
                    UserAuthorization::DEAUTHORIZE_FROM_ONE_DEVICE_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::DeauthorizeFromOneDevice_,
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
                    UserAuthorization::DEAUTHORIZE_FROM_ALL_DEVICES_,
                    ActionRoute::UserAuthorization {
                        user_authorization: UserAuthorization::DeauthorizeFromAllDevices_,
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
                    Channel::GET_ONE_BY_ID_,
                    ActionRoute::Channel {
                        channel: Channel::GetOneById_,
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
                    Channel::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_,
                    ActionRoute::Channel {
                        channel: Channel::GetManyByNameInSubscriptions_,
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
                    Channel::GET_MANY_BY_SUBSCRIPTION_,
                    ActionRoute::Channel {
                        channel: Channel::GetManyBySubscription_,
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
                    Channel::GET_MANY_PUBLIC_BY_NAME_,
                    ActionRoute::Channel {
                        channel: Channel::GetManyPublicByName_,
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
                    Channel::CREATE_,
                    ActionRoute::Channel {
                        channel: Channel::Create_,
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
                    Channel::CHECK_NAME_FOR_EXISTING_,
                    ActionRoute::Channel {
                        channel: Channel::CheckNameForExisting_,
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
                    Channel::CHECK_LINKED_NAME_FOR_EXISTING_,
                    ActionRoute::Channel {
                        channel: Channel::CheckLinkedNameForExisting_,
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
                    ChannelSubscription::CREATE_,
                    ActionRoute::ChannelSubscription {
                        channel_subscription: ChannelSubscription::Create_,
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
    fn process_request(request: Request, environment_configuration: &'static EnvironmentConfiguration, cloned: Arc<Cloned>) -> impl Future<Output = Response> + Send {
        return async move {
            let (parts, mut incoming) = request.into_parts();
            let mut action_inner = ActionInner {
                incoming: &mut incoming,
                parts: &parts,
            };
            let r#match = match cloned.router.at(parts.uri.path()) {
                Result::Ok(r#match_) => r#match_,
                Result::Err(_) => {
                    return Action::<RouteNotFound>::run(&action_inner);
                }
            };
            let action_processor_inner = ActionProcessorInner {
                environment_configuration,
                database_1_postgresql_connection_pool: &cloned.database_1_postgresql_connection_pool,
                database_2_postgresql_connection_pool: &cloned.database_2_postgresql_connection_pool,
            };
            match *r#match.value {
                ActionRoute::UserAuthorization {
                    ref user_authorization,
                } => {
                    match (
                        user_authorization,
                        &parts.method,
                    ) {
                        // Should be GET.
                        (&UserAuthorization::CheckNicknameForExisting, &Method::POST) => {
                            return Action::<UserAuthorization_CheckNicknameForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET.
                        (&UserAuthorization::CheckEmailForExisting, &Method::POST) => {
                            return Action::<UserAuthorization_CheckEmailForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::RegisterByFirstStep, &Method::POST) => {
                            return Action::<UserAuthorization_RegisterByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::RegisterBySecondStep, &Method::POST) => {
                            return Action::<UserAuthorization_RegisterBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::RegisterByLastStep, &Method::POST) => {
                            return Action::<UserAuthorization_RegisterByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::SendEmailForRegister, &Method::POST) => {
                            return Action::<UserAuthorization_SendEmailForRegister>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::AuthorizeByFirstStep, &Method::POST) => {
                            return Action::<UserAuthorization_AuthorizeByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::AuthorizeByLastStep, &Method::POST) => {
                            return Action::<UserAuthorization_AuthorizeByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::SendEmailForAuthorize, &Method::POST) => {
                            return Action::<UserAuthorization_SendEmailForAuthorize>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::ResetPasswordByFirstStep, &Method::POST) => {
                            return Action::<UserAuthorization_ResetPasswordByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::ResetPasswordBySecondStep, &Method::POST) => {
                            return Action::<UserAuthorization_ResetPasswordBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::ResetPasswordByLastStep, &Method::POST) => {
                            return Action::<UserAuthorization_ResetPasswordByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::SendEmailForResetPassword, &Method::POST) => {
                            return Action::<UserAuthorization_SendEmailForResetPassword>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::RefreshAccessToken, &Method::POST) => {
                            return Action::<UserAuthorization_RefreshAccessToken>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::DeauthorizeFromOneDevice, &Method::POST) => {
                            return Action::<UserAuthorization_DeauthorizeFromOneDevice>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization::DeauthorizeFromAllDevices, &Method::POST) => {
                            return Action::<UserAuthorization_DeauthorizeFromAllDevices>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    user_authorization,
                                    &parts.method,
                                ) {
                                    // Should be GET.
                                    (&UserAuthorization::CheckNicknameForExisting_, &Method::POST) => {
                                        return Action::<UserAuthorization_CheckNicknameForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET.
                                    (&UserAuthorization::CheckEmailForExisting_, &Method::POST) => {
                                        return Action::<UserAuthorization_CheckEmailForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::RegisterByFirstStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_RegisterByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::RegisterBySecondStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_RegisterBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::RegisterByLastStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_RegisterByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::SendEmailForRegister_, &Method::POST) => {
                                        return Action::<UserAuthorization_SendEmailForRegister>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::AuthorizeByFirstStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_AuthorizeByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::AuthorizeByLastStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_AuthorizeByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::SendEmailForAuthorize_, &Method::POST) => {
                                        return Action::<UserAuthorization_SendEmailForAuthorize>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::ResetPasswordByFirstStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_ResetPasswordByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::ResetPasswordBySecondStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_ResetPasswordBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::ResetPasswordByLastStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_ResetPasswordByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::SendEmailForResetPassword_, &Method::POST) => {
                                        return Action::<UserAuthorization_SendEmailForResetPassword>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::RefreshAccessToken_, &Method::POST) => {
                                        return Action::<UserAuthorization_RefreshAccessToken>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::DeauthorizeFromOneDevice_, &Method::POST) => {
                                        return Action::<UserAuthorization_DeauthorizeFromOneDevice>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization::DeauthorizeFromAllDevices_, &Method::POST) => {
                                        return Action::<UserAuthorization_DeauthorizeFromAllDevices>::run_(
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
                ActionRoute::Channel {
                    ref channel,
                } => {
                    match (
                        channel,
                        &parts.method,
                    ) {
                        // Should be GET.
                        (&Channel::CheckLinkedNameForExisting, &Method::POST) => {
                            return Action::<Channel_CheckLinkedNameForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET.
                        (&Channel::CheckNameForExisting, &Method::POST) => {
                            return Action::<Channel_CheckNameForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::Create, &Method::POST) => {
                            return Action::<Channel_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET.
                        (&Channel::GetOneById, &Method::POST) => {
                            return Action::<Channel_GetOneById>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET.
                        (&Channel::GetManyByNameInSubscriptions, &Method::POST) => {
                            return Action::<Channel_GetManyByNameInSubscriptions>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET.
                        (&Channel::GetManyBySubscription, &Method::POST) => {
                            return Action::<Channel_GetManyBySubscription>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET.
                        (&Channel::GetManyPublicByName, &Method::POST) => {
                            return Action::<Channel_GetManyPublicByName>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    channel,
                                    &parts.method,
                                ) {
                                    // Should be GET.
                                    (&Channel::CheckLinkedNameForExisting_, &Method::POST) => {
                                        return Action::<Channel_CheckLinkedNameForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET.
                                    (&Channel::CheckNameForExisting_, &Method::POST) => {
                                        return Action::<Channel_CheckNameForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::Create_, &Method::POST) => {
                                        return Action::<Channel_Create>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET.
                                    (&Channel::GetOneById_, &Method::POST) => {
                                        return Action::<Channel_GetOneById>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET.
                                    (&Channel::GetManyByNameInSubscriptions_, &Method::POST) => {
                                        return Action::<Channel_GetManyByNameInSubscriptions>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET.
                                    (&Channel::GetManyBySubscription_, &Method::POST) => {
                                        return Action::<Channel_GetManyBySubscription>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET.
                                    (&Channel::GetManyPublicByName_, &Method::POST) => {
                                        return Action::<Channel_GetManyPublicByName>::run_(
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
                ActionRoute::ChannelSubscription {
                    ref channel_subscription,
                } => {
                    match (
                        channel_subscription,
                        &parts.method,
                    ) {
                        (&ChannelSubscription::Create, &Method::POST) => {
                            return Action::<ChannelSubscription_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    channel_subscription,
                                    &parts.method,
                                ) {
                                    (&ChannelSubscription::Create_, &Method::POST) => {
                                        return Action::<ChannelSubscription_Create>::run_(
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
            return Action::<RouteNotFound>::run(&action_inner);
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
struct Cloned {
    router: Router<ActionRoute>,
    database_1_postgresql_connection_pool: PostgresqlConnectionPool,
    database_2_postgresql_connection_pool: PostgresqlConnectionPool,
}
pub enum ActionRoute {
    UserAuthorization {
        user_authorization: UserAuthorization,
    },
    Channel {
        channel: Channel,
    },
    ChannelSubscription {
        channel_subscription: ChannelSubscription,
    },
}
#[cfg(feature = "action_for_manual_test")]
impl ActionRoute {
    const PART: &'static str = "_";
}
pub enum UserAuthorization {
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
    #[cfg(feature = "action_for_manual_test")]
    CheckNicknameForExisting_,
    #[cfg(feature = "action_for_manual_test")]
    CheckEmailForExisting_,
    #[cfg(feature = "action_for_manual_test")]
    RegisterByFirstStep_,
    #[cfg(feature = "action_for_manual_test")]
    RegisterBySecondStep_,
    #[cfg(feature = "action_for_manual_test")]
    RegisterByLastStep_,
    #[cfg(feature = "action_for_manual_test")]
    SendEmailForRegister_,
    #[cfg(feature = "action_for_manual_test")]
    AuthorizeByFirstStep_,
    #[cfg(feature = "action_for_manual_test")]
    AuthorizeByLastStep_,
    #[cfg(feature = "action_for_manual_test")]
    SendEmailForAuthorize_,
    #[cfg(feature = "action_for_manual_test")]
    ResetPasswordByFirstStep_,
    #[cfg(feature = "action_for_manual_test")]
    ResetPasswordBySecondStep_,
    #[cfg(feature = "action_for_manual_test")]
    ResetPasswordByLastStep_,
    #[cfg(feature = "action_for_manual_test")]
    SendEmailForResetPassword_,
    #[cfg(feature = "action_for_manual_test")]
    RefreshAccessToken_,
    #[cfg(feature = "action_for_manual_test")]
    DeauthorizeFromOneDevice_,
    #[cfg(feature = "action_for_manual_test")]
    DeauthorizeFromAllDevices_,
}
impl UserAuthorization {
    pub const AUTHORIZE_BY_FIRST_STEP: &'static str = "/user_authorization/authorize_by_first_step";
    pub const AUTHORIZE_BY_LAST_STEP: &'static str = "/user_authorization/authorize_by_last_step";
    pub const CHECK_EMAIL_FOR_EXISTING: &'static str = "/user_authorization/check_email_for_existing";
    pub const CHECK_NICKNAME_FOR_EXISTING: &'static str = "/user_authorization/check_nickname_for_existing";
    pub const DEAUTHORIZE_FROM_ALL_DEVICES: &'static str = "/user_authorization/deauthorize_from_all_devices";
    pub const DEAUTHORIZE_FROM_ONE_DEVICE: &'static str = "/user_authorization/deauthorize_from_one_device";
    pub const REFRESH_ACCESS_TOKEN: &'static str = "/user_authorization/refresh_access_token";
    pub const REGISTER_BY_FIRST_STEP: &'static str = "/user_authorization/register_by_first_step";
    pub const REGISTER_BY_LAST_STEP: &'static str = "/user_authorization/register_by_last_step";
    pub const REGISTER_BY_SECOND_STEP: &'static str = "/user_authorization/register_by_second_step";
    pub const RESET_PASSWORD_BY_FIRST_STEP: &'static str = "/user_authorization/reset_password_by_first_step";
    pub const RESET_PASSWORD_BY_LAST_STEP: &'static str = "/user_authorization/reset_password_by_last_step";
    pub const RESET_PASSWORD_BY_SECOND_STEP: &'static str = "/user_authorization/reset_password_by_second_step";
    pub const SEND_EMAIL_FOR_AUTHORIZE: &'static str = "/user_authorization/send_email_for_authorize";
    pub const SEND_EMAIL_FOR_REGISTER: &'static str = "/user_authorization/send_email_for_register";
    pub const SEND_EMAIL_FOR_RESET_PASSWORD: &'static str = "/user_authorization/send_email_for_reset_password";
}
#[cfg(feature = "action_for_manual_test")]
impl UserAuthorization {
    pub const AUTHORIZE_BY_FIRST_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::AUTHORIZE_BY_FIRST_STEP,
        ActionRoute::PART,
    );
    pub const AUTHORIZE_BY_LAST_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::AUTHORIZE_BY_LAST_STEP,
        ActionRoute::PART,
    );
    pub const CHECK_EMAIL_FOR_EXISTING_: &'static str = const_format::concatcp!(
        UserAuthorization::CHECK_EMAIL_FOR_EXISTING,
        ActionRoute::PART,
    );
    pub const CHECK_NICKNAME_FOR_EXISTING_: &'static str = const_format::concatcp!(
        UserAuthorization::CHECK_NICKNAME_FOR_EXISTING,
        ActionRoute::PART,
    );
    pub const DEAUTHORIZE_FROM_ALL_DEVICES_: &'static str = const_format::concatcp!(
        UserAuthorization::DEAUTHORIZE_FROM_ALL_DEVICES,
        ActionRoute::PART,
    );
    pub const DEAUTHORIZE_FROM_ONE_DEVICE_: &'static str = const_format::concatcp!(
        UserAuthorization::DEAUTHORIZE_FROM_ONE_DEVICE,
        ActionRoute::PART,
    );
    pub const REFRESH_ACCESS_TOKEN_: &'static str = const_format::concatcp!(
        UserAuthorization::REFRESH_ACCESS_TOKEN,
        ActionRoute::PART,
    );
    pub const REGISTER_BY_FIRST_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::REGISTER_BY_FIRST_STEP,
        ActionRoute::PART,
    );
    pub const REGISTER_BY_LAST_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::REGISTER_BY_LAST_STEP,
        ActionRoute::PART,
    );
    pub const REGISTER_BY_SECOND_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::REGISTER_BY_SECOND_STEP,
        ActionRoute::PART,
    );
    pub const RESET_PASSWORD_BY_FIRST_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::RESET_PASSWORD_BY_FIRST_STEP,
        ActionRoute::PART,
    );
    pub const RESET_PASSWORD_BY_LAST_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::RESET_PASSWORD_BY_LAST_STEP,
        ActionRoute::PART,
    );
    pub const RESET_PASSWORD_BY_SECOND_STEP_: &'static str = const_format::concatcp!(
        UserAuthorization::RESET_PASSWORD_BY_SECOND_STEP,
        ActionRoute::PART,
    );
    pub const SEND_EMAIL_FOR_AUTHORIZE_: &'static str = const_format::concatcp!(
        UserAuthorization::SEND_EMAIL_FOR_AUTHORIZE,
        ActionRoute::PART,
    );
    pub const SEND_EMAIL_FOR_REGISTER_: &'static str = const_format::concatcp!(
        UserAuthorization::SEND_EMAIL_FOR_REGISTER,
        ActionRoute::PART,
    );
    pub const SEND_EMAIL_FOR_RESET_PASSWORD_: &'static str = const_format::concatcp!(
        UserAuthorization::SEND_EMAIL_FOR_RESET_PASSWORD,
        ActionRoute::PART,
    );
}
pub enum Channel {
    GetOneById,
    GetManyByNameInSubscriptions,
    GetManyBySubscription,
    GetManyPublicByName,
    Create,
    CheckNameForExisting,
    CheckLinkedNameForExisting,
    #[cfg(feature = "action_for_manual_test")]
    GetOneById_,
    #[cfg(feature = "action_for_manual_test")]
    GetManyByNameInSubscriptions_,
    #[cfg(feature = "action_for_manual_test")]
    GetManyBySubscription_,
    #[cfg(feature = "action_for_manual_test")]
    GetManyPublicByName_,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
    #[cfg(feature = "action_for_manual_test")]
    CheckNameForExisting_,
    #[cfg(feature = "action_for_manual_test")]
    CheckLinkedNameForExisting_,
}
impl Channel {
    pub const CHECK_LINKED_NAME_FOR_EXISTING: &'static str = "/channel/check_linked_name_for_existing";
    pub const CHECK_NAME_FOR_EXISTING: &'static str = "/channel/check_name_for_existing";
    pub const CREATE: &'static str = "/channel/create";
    pub const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS: &'static str = "/channel/get_many_by_name_in_subscriptions";
    pub const GET_MANY_BY_SUBSCRIPTION: &'static str = "/channel/get_many_by_subscription";
    pub const GET_MANY_PUBLIC_BY_NAME: &'static str = "/channel/get_many_public_by_name";
    pub const GET_ONE_BY_ID: &'static str = "/channel/get_one_by_id";
}
#[cfg(feature = "action_for_manual_test")]
impl Channel {
    pub const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_: &'static str = const_format::concatcp!(
        Channel::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        ActionRoute::PART,
    );
    pub const GET_MANY_BY_SUBSCRIPTION_: &'static str = const_format::concatcp!(
        Channel::GET_MANY_BY_SUBSCRIPTION,
        ActionRoute::PART,
    );
    pub const GET_MANY_PUBLIC_BY_NAME_: &'static str = const_format::concatcp!(
        Channel::GET_MANY_PUBLIC_BY_NAME,
        ActionRoute::PART,
    );
    pub const GET_ONE_BY_ID_: &'static str = const_format::concatcp!(
        Channel::GET_ONE_BY_ID,
        ActionRoute::PART,
    );
    pub const CREATE_: &'static str = const_format::concatcp!(
        Channel::CREATE,
        ActionRoute::PART,
    );
    pub const CHECK_NAME_FOR_EXISTING_: &'static str = const_format::concatcp!(
        Channel::CHECK_NAME_FOR_EXISTING,
        ActionRoute::PART,
    );
    pub const CHECK_LINKED_NAME_FOR_EXISTING_: &'static str = const_format::concatcp!(
        Channel::CHECK_LINKED_NAME_FOR_EXISTING,
        ActionRoute::PART,
    );
}
pub enum ChannelSubscription {
    Create,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
}
impl ChannelSubscription {
    pub const CREATE: &'static str = "/channel_subscription/create";
}
#[cfg(feature = "action_for_manual_test")]
impl ChannelSubscription {
    pub const CREATE_: &'static str = const_format::concatcp!(
        ChannelSubscription::CREATE,
        ActionRoute::PART,
    );
}
