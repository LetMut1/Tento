mod action_route;
use action_route::{
    ACTION_ROUTE,
    ActionRoute_,
    UserAuthorization_,
    ChannelSubscription_,
    Channel_,
    Channel,
    ChannelSubscription,
    UserAuthorization,
};
use crate::{
    application_layer::functionality::action_processor::{
        user_authorization::{
            authorize_by_first_step::UserAuthorization_AuthorizeByFirstStep,
            authorize_by_last_step::UserAuthorization_AuthorizeByLastStep,
            check_email_for_existing::UserAuthorization_CheckEmailForExisting,
            check_nickname_for_existing::UserAuthorization_CheckNicknameForExisting,
            deauthorize_from_all_devices::UserAuthorization_DeauthorizeFromAllDevices,
            deauthorize_from_one_device::UserAuthorization_DeauthorizeFromOneDevice,
            refresh_access_token::UserAuthorization_RefreshAccessToken,
            register_by_first_step::UserAuthorization_RegisterByFirstStep,
            register_by_last_step::UserAuthorization_RegisterByLastStep,
            register_by_second_step::UserAuthorization_RegisterBySecondStep,
            reset_password_by_first_step::UserAuthorization_ResetPasswordByFirstStep,
            reset_password_by_last_step::UserAuthorization_ResetPasswordByLastStep,
            reset_password_by_second_step::UserAuthorization_ResetPasswordBySecondStep,
            send_email_for_authorize::UserAuthorization_SendEmailForAuthorize,
            send_email_for_register::UserAuthorization_SendEmailForRegister,
            send_email_for_reset_password::UserAuthorization_SendEmailForResetPassword,
        },
        channel::{
            get_many_by_name_in_subscriptions::Channel_GetManyByNameInSubscriptions,
            get_many_by_subscription::Channel_GetManyBySubscription,
            get_many_public_by_name::Channel_GetManyPublicByName,
            get_one_by_id::Channel_GetOneById,
        },
        channel_subscription::create::ChannelSubscription_Create,
        Inner as ActionProcessorInner,
    },
    infrastructure_layer::{
        data::{
            control_type::{
                Request,
                Response,
            },
            environment_configuration::EnvironmentConfiguration,
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
                    _ = signal_interrupt_future => {},
                    _ = signal_terminate_future => {},
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
    fn create_router() -> Result<Router<ActionRoute_>, AggregateError> {
        let mut router = Router::<ActionRoute_>::new();
        router
            .insert(
                ACTION_ROUTE.user_authorization.check_nickname_for_existing,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::CheckNicknameForExisting,
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
                ACTION_ROUTE.user_authorization.check_email_for_existing,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::CheckEmailForExisting,
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
                ACTION_ROUTE.user_authorization.regisgter_by_first_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::RegisterByFirstStep,
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
                ACTION_ROUTE.user_authorization.regisgter_by_second_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::RegisterBySecondStep,
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
                ACTION_ROUTE.user_authorization.regisgter_by_last_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::RegisterByLastStep,
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
                ACTION_ROUTE.user_authorization.send_email_for_register,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::SendEmailForRegister,
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
                ACTION_ROUTE.user_authorization.authorize_by_first_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::AuthorizeByFirstStep,
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
                ACTION_ROUTE.user_authorization.authorize_by_last_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::AuthorizeByLastStep,
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
                ACTION_ROUTE.user_authorization.send_email_for_authorize,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::SendEmailForAuthorize,
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
                ACTION_ROUTE.user_authorization.reset_password_by_first_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::ResetPasswordByFirstStep,
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
                ACTION_ROUTE.user_authorization.reset_password_by_second_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::ResetPasswordBySecondStep,
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
                ACTION_ROUTE.user_authorization.reset_password_by_last_step,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::ResetPasswordByLastStep,
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
                ACTION_ROUTE.user_authorization.send_email_for_reset_password,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::SendEmailForResetPassword,
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
                ACTION_ROUTE.user_authorization.refresh_access_token,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::RefreshAccessToken,
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
                ACTION_ROUTE.user_authorization.deauthorize_from_one_device,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::DeauthorizeFromOneDevice,
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
                ACTION_ROUTE.user_authorization.deauthorize_from_all_devices,
                ActionRoute_::UserAuthorization {
                    user_authorization: UserAuthorization_::DeauthorizeFromAllDevices,
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
                ACTION_ROUTE.channel.get_one_by_id,
                ActionRoute_::Channel {
                    channel: Channel_::GetOneById,
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
                ACTION_ROUTE.channel.get_many_by_name_in_subscription,
                ActionRoute_::Channel {
                    channel: Channel_::GetManyByNameInSubscriptions,
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
                ACTION_ROUTE.channel.get_many_by_subscription,
                ActionRoute_::Channel {
                    channel: Channel_::GetManyBySubscription,
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
                ACTION_ROUTE.channel.get_many_public_by_name,
                ActionRoute_::Channel {
                    channel: Channel_::GetManyPublicByName,
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
                ACTION_ROUTE.channel.create,
                ActionRoute_::Channel {
                    channel: Channel_::Create,
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
                ACTION_ROUTE.channel.check_name_for_existing,
                ActionRoute_::Channel {
                    channel: Channel_::CheckNameForExisting,
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
                ACTION_ROUTE.channel.check_linked_name_for_existing,
                ActionRoute_::Channel {
                    channel: Channel_::CheckLinkedNameForExisting,
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
                ACTION_ROUTE.channel_subscription.create,
                ActionRoute_::ChannelSubscription {
                    channel_subscription: ChannelSubscription_::Create,
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
                    ACTION_ROUTE.user_authorization.check_nickname_for_existing_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::CheckNicknameForExisting_,
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
                    ACTION_ROUTE.user_authorization.check_email_for_existing_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::CheckEmailForExisting_,
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
                    ACTION_ROUTE.user_authorization.regisgter_by_first_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::RegisterByFirstStep_,
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
                    ACTION_ROUTE.user_authorization.regisgter_by_second_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::RegisterBySecondStep_,
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
                    ACTION_ROUTE.user_authorization.regisgter_by_last_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::RegisterByLastStep_,
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
                    ACTION_ROUTE.user_authorization.send_email_for_register_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::SendEmailForRegister_,
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
                    ACTION_ROUTE.user_authorization.authorize_by_first_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::AuthorizeByFirstStep_,
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
                    ACTION_ROUTE.user_authorization.authorize_by_last_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::AuthorizeByLastStep_,
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
                    ACTION_ROUTE.user_authorization.send_email_for_authorize_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::SendEmailForAuthorize_,
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
                    ACTION_ROUTE.user_authorization.reset_password_by_first_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::ResetPasswordByFirstStep_,
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
                    ACTION_ROUTE.user_authorization.reset_password_by_second_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::ResetPasswordBySecondStep_,
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
                    ACTION_ROUTE.user_authorization.reset_password_by_last_step_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::ResetPasswordByLastStep_,
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
                    ACTION_ROUTE.user_authorization.send_email_for_reset_password_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::SendEmailForResetPassword_,
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
                    ACTION_ROUTE.user_authorization.refresh_access_token_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::RefreshAccessToken_,
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
                    ACTION_ROUTE.user_authorization.deauthorize_from_one_device_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::DeauthorizeFromOneDevice_,
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
                    ACTION_ROUTE.user_authorization.deauthorize_from_all_devices_,
                    ActionRoute_::UserAuthorization {
                        user_authorization: UserAuthorization_::DeauthorizeFromAllDevices_,
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
                    ACTION_ROUTE.channel.get_one_by_id_,
                    ActionRoute_::Channel {
                        channel: Channel_::GetOneById_,
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
                    ACTION_ROUTE.channel.get_many_by_name_in_subscription_,
                    ActionRoute_::Channel {
                        channel: Channel_::GetManyByNameInSubscriptions_,
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
                    ACTION_ROUTE.channel.get_many_by_subscription_,
                    ActionRoute_::Channel {
                        channel: Channel_::GetManyBySubscription_,
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
                    ACTION_ROUTE.channel.get_many_public_by_name_,
                    ActionRoute_::Channel {
                        channel: Channel_::GetManyPublicByName_,
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
                    ACTION_ROUTE.channel.create_,
                    ActionRoute_::Channel {
                        channel: Channel_::Create_,
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
                    ACTION_ROUTE.channel.check_name_for_existing_,
                    ActionRoute_::Channel {
                        channel: Channel_::CheckNameForExisting_,
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
                    ACTION_ROUTE.channel.check_linked_name_for_existing_,
                    ActionRoute_::Channel {
                        channel: Channel_::CheckLinkedNameForExisting_,
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
                    ACTION_ROUTE.channel_subscription.create_,
                    ActionRoute_::ChannelSubscription {
                        channel_subscription: ChannelSubscription_::Create_,
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
                    return Action::<RouteNotFound>::run(&action_inner);
                }
            };
            let action_processor_inner = ActionProcessorInner {
                environment_configuration,
                database_1_postgresql_connection_pool: &cloned.database_1_postgresql_connection_pool,
                database_2_postgresql_connection_pool: &cloned.database_2_postgresql_connection_pool,
            };
            match *r#match.value {
                ActionRoute_::UserAuthorization {
                    ref user_authorization,
                } => {
                    match (
                        user_authorization,
                        &parts.method,
                    ) {
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&UserAuthorization_::CheckNicknameForExisting, &Method::POST) => {
                            return Action::<UserAuthorization_CheckNicknameForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&UserAuthorization_::CheckEmailForExisting, &Method::POST) => {
                            return Action::<UserAuthorization_CheckEmailForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::RegisterByFirstStep, &Method::POST) => {
                            return Action::<UserAuthorization_RegisterByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::RegisterBySecondStep, &Method::POST) => {
                            return Action::<UserAuthorization_RegisterBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::RegisterByLastStep, &Method::POST) => {
                            return Action::<UserAuthorization_RegisterByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::SendEmailForRegister, &Method::POST) => {
                            return Action::<UserAuthorization_SendEmailForRegister>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::AuthorizeByFirstStep, &Method::POST) => {
                            return Action::<UserAuthorization_AuthorizeByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::AuthorizeByLastStep, &Method::POST) => {
                            return Action::<UserAuthorization_AuthorizeByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::SendEmailForAuthorize, &Method::POST) => {
                            return Action::<UserAuthorization_SendEmailForAuthorize>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::ResetPasswordByFirstStep, &Method::POST) => {
                            return Action::<UserAuthorization_ResetPasswordByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::ResetPasswordBySecondStep, &Method::POST) => {
                            return Action::<UserAuthorization_ResetPasswordBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::ResetPasswordByLastStep, &Method::POST) => {
                            return Action::<UserAuthorization_ResetPasswordByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::SendEmailForResetPassword, &Method::POST) => {
                            return Action::<UserAuthorization_SendEmailForResetPassword>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::RefreshAccessToken, &Method::POST) => {
                            return Action::<UserAuthorization_RefreshAccessToken>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::DeauthorizeFromOneDevice, &Method::POST) => {
                            return Action::<UserAuthorization_DeauthorizeFromOneDevice>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&UserAuthorization_::DeauthorizeFromAllDevices, &Method::POST) => {
                            return Action::<UserAuthorization_DeauthorizeFromAllDevices>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "manual_testing")]
                            {
                                match (
                                    user_authorization,
                                    &parts.method,
                                ) {
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&UserAuthorization_::CheckNicknameForExisting_, &Method::POST) => {
                                        return Action::<UserAuthorization_CheckNicknameForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&UserAuthorization_::CheckEmailForExisting_, &Method::POST) => {
                                        return Action::<UserAuthorization_CheckEmailForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::RegisterByFirstStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_RegisterByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::RegisterBySecondStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_RegisterBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::RegisterByLastStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_RegisterByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::SendEmailForRegister_, &Method::POST) => {
                                        return Action::<UserAuthorization_SendEmailForRegister>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::AuthorizeByFirstStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_AuthorizeByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::AuthorizeByLastStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_AuthorizeByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::SendEmailForAuthorize_, &Method::POST) => {
                                        return Action::<UserAuthorization_SendEmailForAuthorize>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::ResetPasswordByFirstStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_ResetPasswordByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::ResetPasswordBySecondStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_ResetPasswordBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::ResetPasswordByLastStep_, &Method::POST) => {
                                        return Action::<UserAuthorization_ResetPasswordByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::SendEmailForResetPassword_, &Method::POST) => {
                                        return Action::<UserAuthorization_SendEmailForResetPassword>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::RefreshAccessToken_, &Method::POST) => {
                                        return Action::<UserAuthorization_RefreshAccessToken>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::DeauthorizeFromOneDevice_, &Method::POST) => {
                                        return Action::<UserAuthorization_DeauthorizeFromOneDevice>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&UserAuthorization_::DeauthorizeFromAllDevices_, &Method::POST) => {
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
                ActionRoute_::Channel {
                    ref channel,
                } => {
                    match (
                        channel,
                        &parts.method,
                    ) {
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel_::GetOneById, &Method::POST) => {
                            return Action::<Channel_GetOneById>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel_::GetManyByNameInSubscriptions, &Method::POST) => {
                            return Action::<Channel_GetManyByNameInSubscriptions>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel_::GetManyBySubscription, &Method::POST) => {
                            return Action::<Channel_GetManyBySubscription>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        (&Channel_::GetManyPublicByName, &Method::POST) => {
                            return Action::<Channel_GetManyPublicByName>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "manual_testing")]
                            {
                                match (
                                    channel,
                                    &parts.method,
                                ) {
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel_::GetOneById_, &Method::POST) => {
                                        return Action::<Channel_GetOneById>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel_::GetManyByNameInSubscriptions_, &Method::POST) => {
                                        return Action::<Channel_GetManyByNameInSubscriptions>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel_::GetManyBySubscription_, &Method::POST) => {
                                        return Action::<Channel_GetManyBySubscription>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                                    (&Channel_::GetManyPublicByName_, &Method::POST) => {
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
                ActionRoute_::ChannelSubscription {
                    ref channel_subscription,
                } => {
                    match (
                        channel_subscription,
                        &parts.method,
                    ) {
                        (&ChannelSubscription_::Create, &Method::POST) => {
                            return Action::<ChannelSubscription_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "manual_testing")]
                            {
                                match (
                                    channel_subscription,
                                    &parts.method,
                                ) {
                                    (&ChannelSubscription_::Create_, &Method::POST) => {
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