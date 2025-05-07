#[cfg(not(feature = "postgresql_connection_with_tls"))]
use {
    crate::infrastructure_layer::functionality::service::creator::Creator,
    tokio_postgres::NoTls,
};
use {
    crate::{
        application_layer::functionality::{
            action_processor::{
                Channel_CheckLinkedNameForExisting,
                Channel_CheckNameForExisting,
                Channel_Create,
                Channel_GetManyByNameInSubscriptions,
                Channel_GetManyBySubscription,
                Channel_GetManyPublicByName,
                Channel_GetOneById,
                Channel_RefreshChannelToken,
                ChannelPublication1_Create,
                ChannelPublication1_Delete,
                ChannelPublication1_GetMany,
                ChannelPublication1Commentary_Create,
                ChannelPublication1Commentary_Delete,
                ChannelPublication1Mark_Create,
                ChannelPublication1Mark_Delete,
                ChannelPublication1View_Create,
                ChannelSubscription_Create,
                Channel_Delete,
                Channel_GetManyOwned,
                ChannelSubscription_Delete,
                Inner as ActionProcessorInner,
                User_AuthorizeByFirstStep,
                User_AuthorizeByLastStep,
                User_CheckEmailForExisting,
                User_CheckNicknameForExisting,
                User_DeauthorizeFromAllDevices,
                User_DeauthorizeFromOneDevice,
                User_RefreshAccessToken,
                User_RegisterByFirstStep,
                User_RegisterByLastStep,
                User_RegisterBySecondStep,
                User_ResetPasswordByFirstStep,
                User_ResetPasswordByLastStep,
                User_ResetPasswordBySecondStep,
                User_SendEmailForAuthorize,
                User_SendEmailForRegister,
                User_SendEmailForResetPassword,
            },
            command_processor::RunServer,
        },
        infrastructure_layer::{
            data::{
                aggregate_error::AggregateError,
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
    },
    dedicated::void::Void,
    hyper::{
        Method,
        server::conn::http2::Builder as Http2Builder,
    },
    hyper_util::rt::{
        TokioIo,
        tokio::TokioExecutor,
    },
    matchit::Router,
    std::{
        future::Future,
        sync::Arc,
        time::Duration,
    },
    tokio::{
        net::TcpListener,
        signal::unix::SignalKind,
    },
};
#[cfg(feature = "port_for_manual_test")]
use {
    core::net::SocketAddr,
    hyper::server::conn::http1::Builder as Http1Builder,
};
pub struct HttpServer;
impl HttpServer {
    pub fn run(environment_configuration: &'static EnvironmentConfiguration<RunServer>) -> impl Future<Output = Result<(), AggregateError>> + Send {
        return async move {
            #[cfg(feature = "port_for_manual_test")]
            let http1_socket_address = {
                let mut http1_port_number = environment_configuration.subject.application_server.tcp.socket_address.port();
                if http1_port_number >= u16::MIN && http1_port_number < u16::MAX {
                    http1_port_number += 1;
                } else {
                    http1_port_number -= 1;
                };
                SocketAddr::new(
                    environment_configuration.subject.application_server.tcp.socket_address.ip(),
                    http1_port_number,
                )
            };
            let signal_interrupt_future = Self::create_signal(SignalKind::interrupt())?;
            let signal_terminate_future = Self::create_signal(SignalKind::terminate())?;
            let graceful_shutdown_signal_future_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(
                async move {
                    tokio::select! {
                        _ = signal_interrupt_future => (),
                        _ = signal_terminate_future => (),
                    }
                    return ();
                },
            );
            let mut graceful_shutdown_signal_future_join_handle_ = std::pin::pin!(graceful_shutdown_signal_future_join_handle);
            let cloned = Arc::new(
                Cloned {
                    router: Self::create_router()?,
                    postgresql_connection_pool_database_1: {
                        #[cfg(feature = "postgresql_connection_with_tls")]
                        {
                            todo!();
                        }
                        #[cfg(not(feature = "postgresql_connection_with_tls"))]
                        {
                            Creator::<PostgresqlConnectionPool>::create(
                                &environment_configuration.subject.resource.postgresql.database_1,
                                NoTls,
                            )
                            .await?
                        }
                    },
                    postgresql_connection_pool_database_2: {
                        #[cfg(feature = "postgresql_connection_with_tls")]
                        {
                            todo!();
                        }
                        #[cfg(not(feature = "postgresql_connection_with_tls"))]
                        {
                            Creator::<PostgresqlConnectionPool>::create(
                                &environment_configuration.subject.resource.postgresql.database_2,
                                NoTls,
                            )
                            .await?
                        }
                    },
                    postgresql_connection_pool_database_3: {
                        #[cfg(feature = "postgresql_connection_with_tls")]
                        {
                            todo!();
                        }
                        #[cfg(not(feature = "postgresql_connection_with_tls"))]
                        {
                            Creator::<PostgresqlConnectionPool>::create(
                                &environment_configuration.subject.resource.postgresql.database_3,
                                NoTls,
                            )
                            .await?
                        }
                    },
                    postgresql_connection_pool_database_4: {
                        #[cfg(feature = "postgresql_connection_with_tls")]
                        {
                            todo!();
                        }
                        #[cfg(not(feature = "postgresql_connection_with_tls"))]
                        {
                            Creator::<PostgresqlConnectionPool>::create(
                                &environment_configuration.subject.resource.postgresql.database_4,
                                NoTls,
                            )
                            .await?
                        }
                    },
                },
            );
            'a: loop {
                let cloned_ = cloned.clone();
                let mut graceful_shutdown_signal_future_join_handle__ = graceful_shutdown_signal_future_join_handle_.as_mut();
                #[cfg(feature = "port_for_manual_test")]
                let http1_tcp_listener = crate::result_return_logic!(
                    TcpListener::bind(&http1_socket_address).await
                );
                let http2_tcp_listener = crate::result_return_logic!(
                    TcpListener::bind(&environment_configuration.subject.application_server.tcp.socket_address).await
                );
                if let Result::Err(aggregate_error) = async move {
                    #[cfg(feature = "port_for_manual_test")]
                    let http1_builder = Http1Builder::new();
                    let mut http2_builder = Http2Builder::new(TokioExecutor::new()).max_local_error_reset_streams(Option::Some(128));
                    http2_builder
                        .auto_date_header(false)
                        .max_header_list_size(environment_configuration.subject.application_server.http.maximum_header_list_size)
                        .adaptive_window(environment_configuration.subject.application_server.http.adaptive_window)
                        .initial_connection_window_size(Option::Some(environment_configuration.subject.application_server.http.connection_window_size))
                        .initial_stream_window_size(Option::Some(environment_configuration.subject.application_server.http.stream_window_size))
                        .max_concurrent_streams(Option::None)
                        .max_frame_size(Option::Some(environment_configuration.subject.application_server.http.maximum_frame_size))
                        .max_send_buf_size(environment_configuration.subject.application_server.http.maximum_sending_buffer_size as usize);
                    if environment_configuration.subject.application_server.http.enable_connect_protocol {
                        http2_builder.enable_connect_protocol();
                    };
                    match environment_configuration.subject.application_server.http.keepalive {
                        Option::Some(ref keepalive) => {
                            http2_builder
                                .keep_alive_interval(Option::Some(Duration::from_secs(keepalive.interval_duration)))
                                .keep_alive_timeout(Duration::from_secs(keepalive.timeout_duration))
                        }
                        Option::None => http2_builder.keep_alive_interval(Option::None),
                    };
                    match environment_configuration.subject.application_server.http.maximum_pending_accept_reset_streams {
                        Option::Some(maximum_pending_accept_reset_streams) => http2_builder.max_pending_accept_reset_streams(Option::Some(maximum_pending_accept_reset_streams)),
                        Option::None => http2_builder.max_pending_accept_reset_streams(Option::None),
                    };
                    if let Option::Some(ref _tls) = environment_configuration.subject.application_server.http.tls {
                        todo!("// TODO ssl_protocolsTLSv1 TLSv1.1 TLSv1.2 TLSv1.3;  ssl_ciphers HIGH:!aNULL:!MD5;")
                    }
                    'b: loop {
                        #[cfg(not(feature = "port_for_manual_test"))]
                        let tcp_accepting_future = http2_tcp_listener.accept();
                        #[cfg(feature = "port_for_manual_test")]
                        let tcp_accepting_future = async {
                            return tokio::select! {
                                result = http1_tcp_listener.accept() => result,
                                result = http2_tcp_listener.accept() => result,
                            };
                        };
                        tokio::select! {
                            biased;
                            _ = graceful_shutdown_signal_future_join_handle__.as_mut() => {
                                break 'b;
                            },
                            tcp_stream = tcp_accepting_future => {
                                let tcp_stream_ = match tcp_stream {
                                    Result::Ok((tcp_stream__, _)) => tcp_stream__,
                                    Result::Err(error) => {
                                        Spawner::<TokioNonBlockingTask>::spawn_into_background(
                                            async move {
                                                Logger::<AggregateError>::log(
                                                    &crate::new_runtime!(error),
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
                                    let socket_address_port = crate::result_return_logic!(tcp_stream_.local_addr());
                                    if http1_socket_address.port() == socket_address_port.port() {
                                        let serving_connection_future = http1_builder.serve_connection(
                                            TokioIo::new(tcp_stream_),
                                            service_fn,
                                        );
                                        Spawner::<TokioNonBlockingTask>::spawn_into_background(
                                            async move {
                                                return crate::result_into_runtime!(serving_connection_future.await);
                                            },
                                        );
                                    } else {
                                        let serving_connection_future = http2_builder.serve_connection(
                                            TokioIo::new(tcp_stream_),
                                            service_fn,
                                        );
                                        Spawner::<TokioNonBlockingTask>::spawn_into_background(
                                            async move {
                                                return crate::result_into_runtime!(serving_connection_future.await);
                                            },
                                        );
                                    };
                                }
                                #[cfg(not(feature = "port_for_manual_test"))]
                                {
                                    let serving_connection_future = http2_builder.serve_connection(
                                        TokioIo::new(tcp_stream_),
                                        service_fn,
                                    );
                                    Spawner::<TokioNonBlockingTask>::spawn_into_background(
                                        async move {
                                            return crate::result_into_runtime!(serving_connection_future.await);
                                        },
                                    );
                                }
                                continue 'b;
                            },
                        }
                    }
                    let mut interval = tokio::time::interval(Duration::from_secs(1));
                    '_b: for seconds_quantity in (1..=15).rev() {
                        interval.tick().await;
                        println!("Remaining time in seconds before shutdown: {},", seconds_quantity);
                    }
                    return Result::<_, AggregateError>::Ok(());
                }
                .await
                {
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
        crate::result_return_logic!(
            router
            .insert(
                User::CHECK_NICKNAME_FOR_EXISTING,
                ActionRoute::User(User::CheckNicknameForExisting)
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::CHECK_EMAIL_FOR_EXISTING,
                ActionRoute::User(User::CheckEmailForExisting)
            )
        );
        crate::result_return_logic!(router

            .insert(
                User::REGISTER_BY_FIRST_STEP,
                ActionRoute::User(User::RegisterByFirstStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::REGISTER_BY_SECOND_STEP,
                ActionRoute::User(User::RegisterBySecondStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::REGISTER_BY_LAST_STEP,
                ActionRoute::User(User::RegisterByLastStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::SEND_EMAIL_FOR_REGISTER,
                ActionRoute::User(User::SendEmailForRegister),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::AUTHORIZE_BY_FIRST_STEP,
                ActionRoute::User(User::AuthorizeByFirstStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::AUTHORIZE_BY_LAST_STEP,
                ActionRoute::User(User::AuthorizeByLastStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::SEND_EMAIL_FOR_AUTHORIZE,
                ActionRoute::User(User::SendEmailForAuthorize),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::RESET_PASSWORD_BY_FIRST_STEP,
                ActionRoute::User(User::ResetPasswordByFirstStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::RESET_PASSWORD_BY_SECOND_STEP,
                ActionRoute::User(User::ResetPasswordBySecondStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::RESET_PASSWORD_BY_LAST_STEP,
                ActionRoute::User(User::ResetPasswordByLastStep),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::SEND_EMAIL_FOR_RESET_PASSWORD,
                ActionRoute::User(User::SendEmailForResetPassword),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::REFRESH_ACCESS_TOKEN,
                ActionRoute::User(User::RefreshAccessToken),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::DEAUTHORIZE_FROM_ONE_DEVICE,
                ActionRoute::User(User::DeauthorizeFromOneDevice),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                User::DEAUTHORIZE_FROM_ALL_DEVICES,
                ActionRoute::User(User::DeauthorizeFromAllDevices),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::GET_ONE_BY_ID,
                ActionRoute::Channel(Channel::GetOneById),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
                ActionRoute::Channel(Channel::GetManyByNameInSubscriptions),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::GET_MANY_BY_SUBSCRIPTION,
                ActionRoute::Channel(Channel::GetManyBySubscription),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::GET_MANY_PUBLIC_BY_NAME,
                ActionRoute::Channel(Channel::GetManyPublicByName),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::CREATE,
                ActionRoute::Channel(Channel::Create),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::CHECK_NAME_FOR_EXISTING,
                ActionRoute::Channel(Channel::CheckNameForExisting),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::CHECK_LINKED_NAME_FOR_EXISTING,
                ActionRoute::Channel(Channel::CheckLinkedNameForExisting),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::DELETE,
                ActionRoute::Channel(Channel::Delete),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::GET_MANY_OWNED,
                ActionRoute::Channel(Channel::GetManyOwned),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                Channel::REFRESH_CHANNEL_TOKEN,
                ActionRoute::Channel(Channel::RefreshChannelToken),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelSubscription::CREATE,
                ActionRoute::ChannelSubscription(ChannelSubscription::Create),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelSubscription::DELETE,
                ActionRoute::ChannelSubscription(ChannelSubscription::Delete),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1::CREATE,
                ActionRoute::ChannelPublication1(ChannelPublication1::Create),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1::DELETE,
                ActionRoute::ChannelPublication1(ChannelPublication1::Delete),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1::GET_MANY,
                ActionRoute::ChannelPublication1(ChannelPublication1::GetMany),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1Mark::CREATE,
                ActionRoute::ChannelPublication1Mark(ChannelPublication1Mark::Create),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1Mark::DELETE,
                ActionRoute::ChannelPublication1Mark(ChannelPublication1Mark::Delete),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1View::CREATE,
                ActionRoute::ChannelPublication1View(ChannelPublication1View::Create),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1Commentary::CREATE,
                ActionRoute::ChannelPublication1Commentary(ChannelPublication1Commentary::Create),
            )
        );
        crate::result_return_logic!(
            router
            .insert(
                ChannelPublication1Commentary::DELETE,
                ActionRoute::ChannelPublication1Commentary(ChannelPublication1Commentary::Delete),
            )
        );
        #[cfg(feature = "action_for_manual_test")]
        {
            crate::result_return_logic!(
                router
                .insert(
                    User::CHECK_NICKNAME_FOR_EXISTING_,
                    ActionRoute::User(User::CheckNicknameForExisting_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::CHECK_EMAIL_FOR_EXISTING_,
                    ActionRoute::User(User::CheckEmailForExisting_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::REGISTER_BY_FIRST_STEP_,
                    ActionRoute::User(User::RegisterByFirstStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::REGISTER_BY_SECOND_STEP_,
                    ActionRoute::User(User::RegisterBySecondStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::REGISTER_BY_LAST_STEP_,
                    ActionRoute::User(User::RegisterByLastStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::SEND_EMAIL_FOR_REGISTER_,
                    ActionRoute::User(User::SendEmailForRegister_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::AUTHORIZE_BY_FIRST_STEP_,
                    ActionRoute::User(User::AuthorizeByFirstStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::AUTHORIZE_BY_LAST_STEP_,
                    ActionRoute::User(User::AuthorizeByLastStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::SEND_EMAIL_FOR_AUTHORIZE_,
                    ActionRoute::User(User::SendEmailForAuthorize_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::RESET_PASSWORD_BY_FIRST_STEP_,
                    ActionRoute::User(User::ResetPasswordByFirstStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::RESET_PASSWORD_BY_SECOND_STEP_,
                    ActionRoute::User(User::ResetPasswordBySecondStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::RESET_PASSWORD_BY_LAST_STEP_,
                    ActionRoute::User(User::ResetPasswordByLastStep_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::SEND_EMAIL_FOR_RESET_PASSWORD_,
                    ActionRoute::User(User::SendEmailForResetPassword_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::REFRESH_ACCESS_TOKEN_,
                    ActionRoute::User(User::RefreshAccessToken_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::DEAUTHORIZE_FROM_ONE_DEVICE_,
                    ActionRoute::User(User::DeauthorizeFromOneDevice_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    User::DEAUTHORIZE_FROM_ALL_DEVICES_,
                    ActionRoute::User(User::DeauthorizeFromAllDevices_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::GET_ONE_BY_ID_,
                    ActionRoute::Channel(Channel::GetOneById_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_,
                    ActionRoute::Channel(Channel::GetManyByNameInSubscriptions_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::GET_MANY_BY_SUBSCRIPTION_,
                    ActionRoute::Channel(Channel::GetManyBySubscription_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::GET_MANY_PUBLIC_BY_NAME_,
                    ActionRoute::Channel(Channel::GetManyPublicByName_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::CREATE_,
                    ActionRoute::Channel(Channel::Create_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::CHECK_NAME_FOR_EXISTING_,
                    ActionRoute::Channel(Channel::CheckNameForExisting_),
                )
            );
            crate::result_return_logic!(router
                .insert(
                    Channel::CHECK_LINKED_NAME_FOR_EXISTING_,
                    ActionRoute::Channel(Channel::CheckLinkedNameForExisting_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::DELETE_,
                    ActionRoute::Channel(Channel::Delete_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::GET_MANY_OWNED_,
                    ActionRoute::Channel(Channel::GetManyOwned_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    Channel::REFRESH_CHANNEL_TOKEN_,
                    ActionRoute::Channel(Channel::RefreshChannelToken_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelSubscription::CREATE_,
                    ActionRoute::ChannelSubscription(ChannelSubscription::Create_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelSubscription::DELETE_,
                    ActionRoute::ChannelSubscription(ChannelSubscription::Delete_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1::CREATE_,
                    ActionRoute::ChannelPublication1(ChannelPublication1::Create_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1::DELETE_,
                    ActionRoute::ChannelPublication1(ChannelPublication1::Delete_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1::GET_MANY_,
                    ActionRoute::ChannelPublication1(ChannelPublication1::GetMany_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1Mark::CREATE_,
                    ActionRoute::ChannelPublication1Mark(ChannelPublication1Mark::Create_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1Mark::DELETE_,
                    ActionRoute::ChannelPublication1Mark(ChannelPublication1Mark::Delete_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1View::CREATE_,
                    ActionRoute::ChannelPublication1View(ChannelPublication1View::Create_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1Commentary::CREATE_,
                    ActionRoute::ChannelPublication1Commentary(ChannelPublication1Commentary::Create_),
                )
            );
            crate::result_return_logic!(
                router
                .insert(
                    ChannelPublication1Commentary::DELETE_,
                    ActionRoute::ChannelPublication1Commentary(ChannelPublication1Commentary::Delete_),
                )
            );
        }
        return Result::Ok(router);
    }
    fn process_request(request: Request, environment_configuration: &'static EnvironmentConfiguration<RunServer>, cloned: Arc<Cloned>) -> impl Future<Output = Response> + Send {
        return async move {
            let (parts, mut incoming) = request.into_parts();
            let mut action_inner = ActionInner {
                incoming: &mut incoming,
                parts: &parts,
            };
            let r#match = match cloned.router.at(parts.uri.path()) {
                Result::Ok(r#match_) => r#match_,
                Result::Err(_) => return Action::<RouteNotFound>::run(&action_inner),
            };
            let action_processor_inner = ActionProcessorInner {
                environment_configuration,
                postgresql_connection_pool_database_1: &cloned.postgresql_connection_pool_database_1,
                postgresql_connection_pool_database_2: &cloned.postgresql_connection_pool_database_2,
                postgresql_connection_pool_database_3: &cloned.postgresql_connection_pool_database_3,
                postgresql_connection_pool_database_4: &cloned.postgresql_connection_pool_database_4,
            };
            match *r#match.value {
                ActionRoute::User(ref user) => {
                    match (
                        user,
                        &parts.method,
                    ) {
                        (&User::CheckNicknameForExisting, &Method::POST) => {
                            return Action::<User_CheckNicknameForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::CheckEmailForExisting, &Method::POST) => {
                            return Action::<User_CheckEmailForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::RegisterByFirstStep, &Method::POST) => {
                            return Action::<User_RegisterByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::RegisterBySecondStep, &Method::POST) => {
                            return Action::<User_RegisterBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::RegisterByLastStep, &Method::POST) => {
                            return Action::<User_RegisterByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::SendEmailForRegister, &Method::POST) => {
                            return Action::<User_SendEmailForRegister>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::AuthorizeByFirstStep, &Method::POST) => {
                            return Action::<User_AuthorizeByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::AuthorizeByLastStep, &Method::POST) => {
                            return Action::<User_AuthorizeByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::SendEmailForAuthorize, &Method::POST) => {
                            return Action::<User_SendEmailForAuthorize>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::ResetPasswordByFirstStep, &Method::POST) => {
                            return Action::<User_ResetPasswordByFirstStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::ResetPasswordBySecondStep, &Method::POST) => {
                            return Action::<User_ResetPasswordBySecondStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::ResetPasswordByLastStep, &Method::POST) => {
                            return Action::<User_ResetPasswordByLastStep>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::SendEmailForResetPassword, &Method::POST) => {
                            return Action::<User_SendEmailForResetPassword>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::RefreshAccessToken, &Method::POST) => {
                            return Action::<User_RefreshAccessToken>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::DeauthorizeFromOneDevice, &Method::POST) => {
                            return Action::<User_DeauthorizeFromOneDevice>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&User::DeauthorizeFromAllDevices, &Method::POST) => {
                            return Action::<User_DeauthorizeFromAllDevices>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    user,
                                    &parts.method,
                                ) {
                                    (&User::CheckNicknameForExisting_, &Method::POST) => {
                                        return Action::<User_CheckNicknameForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::CheckEmailForExisting_, &Method::POST) => {
                                        return Action::<User_CheckEmailForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::RegisterByFirstStep_, &Method::POST) => {
                                        return Action::<User_RegisterByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::RegisterBySecondStep_, &Method::POST) => {
                                        return Action::<User_RegisterBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::RegisterByLastStep_, &Method::POST) => {
                                        return Action::<User_RegisterByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::SendEmailForRegister_, &Method::POST) => {
                                        return Action::<User_SendEmailForRegister>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::AuthorizeByFirstStep_, &Method::POST) => {
                                        return Action::<User_AuthorizeByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::AuthorizeByLastStep_, &Method::POST) => {
                                        return Action::<User_AuthorizeByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::SendEmailForAuthorize_, &Method::POST) => {
                                        return Action::<User_SendEmailForAuthorize>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::ResetPasswordByFirstStep_, &Method::POST) => {
                                        return Action::<User_ResetPasswordByFirstStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::ResetPasswordBySecondStep_, &Method::POST) => {
                                        return Action::<User_ResetPasswordBySecondStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::ResetPasswordByLastStep_, &Method::POST) => {
                                        return Action::<User_ResetPasswordByLastStep>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::SendEmailForResetPassword_, &Method::POST) => {
                                        return Action::<User_SendEmailForResetPassword>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::RefreshAccessToken_, &Method::POST) => {
                                        return Action::<User_RefreshAccessToken>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::DeauthorizeFromOneDevice_, &Method::POST) => {
                                        return Action::<User_DeauthorizeFromOneDevice>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&User::DeauthorizeFromAllDevices_, &Method::POST) => {
                                        return Action::<User_DeauthorizeFromAllDevices>::run_(
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
                ActionRoute::Channel(ref channel) => {
                    match (
                        channel,
                        &parts.method,
                    ) {
                        (&Channel::CheckLinkedNameForExisting, &Method::POST) => {
                            return Action::<Channel_CheckLinkedNameForExisting>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
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
                        (&Channel::GetOneById, &Method::POST) => {
                            return Action::<Channel_GetOneById>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::GetManyByNameInSubscriptions, &Method::POST) => {
                            return Action::<Channel_GetManyByNameInSubscriptions>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::GetManyBySubscription, &Method::POST) => {
                            return Action::<Channel_GetManyBySubscription>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::GetManyPublicByName, &Method::POST) => {
                            return Action::<Channel_GetManyPublicByName>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::Delete, &Method::POST) => {
                            return Action::<Channel_Delete>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::GetManyOwned, &Method::POST) => {
                            return Action::<Channel_GetManyOwned>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&Channel::RefreshChannelToken, &Method::POST) => {
                            return Action::<Channel_RefreshChannelToken>::run(
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
                                    (&Channel::CheckLinkedNameForExisting_, &Method::POST) => {
                                        return Action::<Channel_CheckLinkedNameForExisting>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
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
                                    (&Channel::GetOneById_, &Method::POST) => {
                                        return Action::<Channel_GetOneById>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::GetManyByNameInSubscriptions_, &Method::POST) => {
                                        return Action::<Channel_GetManyByNameInSubscriptions>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::GetManyBySubscription_, &Method::POST) => {
                                        return Action::<Channel_GetManyBySubscription>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::GetManyPublicByName_, &Method::POST) => {
                                        return Action::<Channel_GetManyPublicByName>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::Delete_, &Method::POST) => {
                                        return Action::<Channel_Delete>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::GetManyOwned_, &Method::POST) => {
                                        return Action::<Channel_GetManyOwned>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&Channel::RefreshChannelToken_, &Method::POST) => {
                                        return Action::<Channel_RefreshChannelToken>::run_(
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
                ActionRoute::ChannelSubscription(ref channel_subscription) => {
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
                        (&ChannelSubscription::Delete, &Method::POST) => {
                            return Action::<ChannelSubscription_Delete>::run(
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
                                    (&ChannelSubscription::Delete_, &Method::POST) => {
                                        return Action::<ChannelSubscription_Delete>::run_(
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
                ActionRoute::ChannelPublication1(ref channel_publication1) => {
                    match (
                        channel_publication1,
                        &parts.method,
                    ) {
                        (&ChannelPublication1::Create, &Method::POST) => {
                            return Action::<ChannelPublication1_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ChannelPublication1::Delete, &Method::POST) => {
                            return Action::<ChannelPublication1_Delete>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ChannelPublication1::GetMany, &Method::POST) => {
                            return Action::<ChannelPublication1_GetMany>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    channel_publication1,
                                    &parts.method,
                                ) {
                                    (&ChannelPublication1::Create_, &Method::POST) => {
                                        return Action::<ChannelPublication1_Create>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ChannelPublication1::Delete_, &Method::POST) => {
                                        return Action::<ChannelPublication1_Delete>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ChannelPublication1::GetMany_, &Method::POST) => {
                                        return Action::<ChannelPublication1_GetMany>::run_(
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
                ActionRoute::ChannelPublication1Mark(ref channel_publication1_mark) => {
                    match (
                        channel_publication1_mark,
                        &parts.method,
                    ) {
                        (&ChannelPublication1Mark::Create, &Method::POST) => {
                            return Action::<ChannelPublication1Mark_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ChannelPublication1Mark::Delete, &Method::POST) => {
                            return Action::<ChannelPublication1Mark_Delete>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    channel_publication1_mark,
                                    &parts.method,
                                ) {
                                    (&ChannelPublication1Mark::Create_, &Method::POST) => {
                                        return Action::<ChannelPublication1Mark_Create>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ChannelPublication1Mark::Delete_, &Method::POST) => {
                                        return Action::<ChannelPublication1Mark_Delete>::run_(
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
                ActionRoute::ChannelPublication1View(ref channel_publication1_view) => {
                    match (
                        channel_publication1_view,
                        &parts.method,
                    ) {
                        (&ChannelPublication1View::Create, &Method::POST) => {
                            return Action::<ChannelPublication1View_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    channel_publication1_view,
                                    &parts.method,
                                ) {
                                    (&ChannelPublication1View::Create_, &Method::POST) => {
                                        return Action::<ChannelPublication1View_Create>::run_(
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
                ActionRoute::ChannelPublication1Commentary(ref channel_publication1_commentary) => {
                    match (
                        channel_publication1_commentary,
                        &parts.method,
                    ) {
                        (&ChannelPublication1Commentary::Create, &Method::POST) => {
                            return Action::<ChannelPublication1Commentary_Create>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        (&ChannelPublication1Commentary::Delete, &Method::POST) => {
                            return Action::<ChannelPublication1Commentary_Delete>::run(
                                &mut action_inner,
                                &action_processor_inner,
                            )
                            .await;
                        }
                        _ => {
                            #[cfg(feature = "action_for_manual_test")]
                            {
                                match (
                                    channel_publication1_commentary,
                                    &parts.method,
                                ) {
                                    (&ChannelPublication1Commentary::Create_, &Method::POST) => {
                                        return Action::<ChannelPublication1Commentary_Create>::run_(
                                            &mut action_inner,
                                            &action_processor_inner,
                                        )
                                        .await;
                                    }
                                    (&ChannelPublication1Commentary::Delete_, &Method::POST) => {
                                        return Action::<ChannelPublication1Commentary_Delete>::run_(
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
    fn create_signal(signal_kind: SignalKind) -> Result<impl Future<Output = ()> + Send + use<>, AggregateError> {
        let mut signal = crate::result_return_logic!(
            tokio::signal::unix::signal(signal_kind)
        );
        let signal_future = async move {
            signal.recv().await;
            return ();
        };
        return Result::Ok(signal_future);
    }
}
struct Cloned {
    router: Router<ActionRoute>,
    postgresql_connection_pool_database_1: PostgresqlConnectionPool,
    postgresql_connection_pool_database_2: PostgresqlConnectionPool,
    postgresql_connection_pool_database_3: PostgresqlConnectionPool,
    postgresql_connection_pool_database_4: PostgresqlConnectionPool,
}
pub enum ActionRoute {
    User(User),
    Channel(Channel),
    ChannelSubscription(ChannelSubscription),
    ChannelPublication1(ChannelPublication1),
    ChannelPublication1Mark(ChannelPublication1Mark),
    ChannelPublication1View(ChannelPublication1View),
    ChannelPublication1Commentary(ChannelPublication1Commentary),
}
#[cfg(feature = "action_for_manual_test")]
impl ActionRoute {
    const PART: &'static str = "_";
}
pub enum User {
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
impl User {
    pub const AUTHORIZE_BY_FIRST_STEP: &'static str = "/user/authorize_by_first_step";
    pub const AUTHORIZE_BY_LAST_STEP: &'static str = "/user/authorize_by_last_step";
    pub const CHECK_EMAIL_FOR_EXISTING: &'static str = "/user/check_email_for_existing";
    pub const CHECK_NICKNAME_FOR_EXISTING: &'static str = "/user/check_nickname_for_existing";
    pub const DEAUTHORIZE_FROM_ALL_DEVICES: &'static str = "/user/deauthorize_from_all_devices";
    pub const DEAUTHORIZE_FROM_ONE_DEVICE: &'static str = "/user/deauthorize_from_one_device";
    pub const REFRESH_ACCESS_TOKEN: &'static str = "/user/refresh_access_token";
    pub const REGISTER_BY_FIRST_STEP: &'static str = "/user/register_by_first_step";
    pub const REGISTER_BY_LAST_STEP: &'static str = "/user/register_by_last_step";
    pub const REGISTER_BY_SECOND_STEP: &'static str = "/user/register_by_second_step";
    pub const RESET_PASSWORD_BY_FIRST_STEP: &'static str = "/user/reset_password_by_first_step";
    pub const RESET_PASSWORD_BY_LAST_STEP: &'static str = "/user/reset_password_by_last_step";
    pub const RESET_PASSWORD_BY_SECOND_STEP: &'static str = "/user/reset_password_by_second_step";
    pub const SEND_EMAIL_FOR_AUTHORIZE: &'static str = "/user/send_email_for_authorize";
    pub const SEND_EMAIL_FOR_REGISTER: &'static str = "/user/send_email_for_register";
    pub const SEND_EMAIL_FOR_RESET_PASSWORD: &'static str = "/user/send_email_for_reset_password";
}
#[cfg(feature = "action_for_manual_test")]
impl User {
    pub const AUTHORIZE_BY_FIRST_STEP_: &'static str = const_format::concatcp!(
        User::AUTHORIZE_BY_FIRST_STEP,
        ActionRoute::PART,
    );
    pub const AUTHORIZE_BY_LAST_STEP_: &'static str = const_format::concatcp!(
        User::AUTHORIZE_BY_LAST_STEP,
        ActionRoute::PART,
    );
    pub const CHECK_EMAIL_FOR_EXISTING_: &'static str = const_format::concatcp!(
        User::CHECK_EMAIL_FOR_EXISTING,
        ActionRoute::PART,
    );
    pub const CHECK_NICKNAME_FOR_EXISTING_: &'static str = const_format::concatcp!(
        User::CHECK_NICKNAME_FOR_EXISTING,
        ActionRoute::PART,
    );
    pub const DEAUTHORIZE_FROM_ALL_DEVICES_: &'static str = const_format::concatcp!(
        User::DEAUTHORIZE_FROM_ALL_DEVICES,
        ActionRoute::PART,
    );
    pub const DEAUTHORIZE_FROM_ONE_DEVICE_: &'static str = const_format::concatcp!(
        User::DEAUTHORIZE_FROM_ONE_DEVICE,
        ActionRoute::PART,
    );
    pub const REFRESH_ACCESS_TOKEN_: &'static str = const_format::concatcp!(
        User::REFRESH_ACCESS_TOKEN,
        ActionRoute::PART,
    );
    pub const REGISTER_BY_FIRST_STEP_: &'static str = const_format::concatcp!(
        User::REGISTER_BY_FIRST_STEP,
        ActionRoute::PART,
    );
    pub const REGISTER_BY_LAST_STEP_: &'static str = const_format::concatcp!(
        User::REGISTER_BY_LAST_STEP,
        ActionRoute::PART,
    );
    pub const REGISTER_BY_SECOND_STEP_: &'static str = const_format::concatcp!(
        User::REGISTER_BY_SECOND_STEP,
        ActionRoute::PART,
    );
    pub const RESET_PASSWORD_BY_FIRST_STEP_: &'static str = const_format::concatcp!(
        User::RESET_PASSWORD_BY_FIRST_STEP,
        ActionRoute::PART,
    );
    pub const RESET_PASSWORD_BY_LAST_STEP_: &'static str = const_format::concatcp!(
        User::RESET_PASSWORD_BY_LAST_STEP,
        ActionRoute::PART,
    );
    pub const RESET_PASSWORD_BY_SECOND_STEP_: &'static str = const_format::concatcp!(
        User::RESET_PASSWORD_BY_SECOND_STEP,
        ActionRoute::PART,
    );
    pub const SEND_EMAIL_FOR_AUTHORIZE_: &'static str = const_format::concatcp!(
        User::SEND_EMAIL_FOR_AUTHORIZE,
        ActionRoute::PART,
    );
    pub const SEND_EMAIL_FOR_REGISTER_: &'static str = const_format::concatcp!(
        User::SEND_EMAIL_FOR_REGISTER,
        ActionRoute::PART,
    );
    pub const SEND_EMAIL_FOR_RESET_PASSWORD_: &'static str = const_format::concatcp!(
        User::SEND_EMAIL_FOR_RESET_PASSWORD,
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
    Delete,
    GetManyOwned,
    RefreshChannelToken,
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
    #[cfg(feature = "action_for_manual_test")]
    Delete_,
    #[cfg(feature = "action_for_manual_test")]
    GetManyOwned_,
    #[cfg(feature = "action_for_manual_test")]
    RefreshChannelToken_,
}
impl Channel {
    pub const CHECK_LINKED_NAME_FOR_EXISTING: &'static str = "/channel/check_linked_name_for_existing";
    pub const CHECK_NAME_FOR_EXISTING: &'static str = "/channel/check_name_for_existing";
    pub const CREATE: &'static str = "/channel/create";
    pub const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS: &'static str = "/channel/get_many_by_name_in_subscriptions";
    pub const GET_MANY_BY_SUBSCRIPTION: &'static str = "/channel/get_many_by_subscription";
    pub const GET_MANY_PUBLIC_BY_NAME: &'static str = "/channel/get_many_public_by_name";
    pub const GET_ONE_BY_ID: &'static str = "/channel/get_one_by_id";
    pub const DELETE: &'static str = "/channel/delete";
    pub const GET_MANY_OWNED: &'static str = "/channel/get_many_owned";
    pub const REFRESH_CHANNEL_TOKEN: &'static str = "/channel/refresh_channel_token";
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
    pub const DELETE_: &'static str = const_format::concatcp!(
        Channel::DELETE,
        ActionRoute::PART,
    );
    pub const GET_MANY_OWNED_: &'static str = const_format::concatcp!(
        Channel::GET_MANY_OWNED,
        ActionRoute::PART,
    );
    pub const REFRESH_CHANNEL_TOKEN_: &'static str = const_format::concatcp!(
        Channel::REFRESH_CHANNEL_TOKEN,
        ActionRoute::PART,
    );
}
pub enum ChannelSubscription {
    Create,
    Delete,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
    #[cfg(feature = "action_for_manual_test")]
    Delete_,
}
impl ChannelSubscription {
    pub const CREATE: &'static str = "/channel_subscription/create";
    pub const DELETE: &'static str = "/channel_subscription/delete";
}
#[cfg(feature = "action_for_manual_test")]
impl ChannelSubscription {
    pub const CREATE_: &'static str = const_format::concatcp!(
        ChannelSubscription::CREATE,
        ActionRoute::PART,
    );
    pub const DELETE_: &'static str = const_format::concatcp!(
        ChannelSubscription::DELETE,
        ActionRoute::PART,
    );
}
pub enum ChannelPublication1 {
    Create,
    Delete,
    GetMany,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
    #[cfg(feature = "action_for_manual_test")]
    Delete_,
    #[cfg(feature = "action_for_manual_test")]
    GetMany_,
}
impl ChannelPublication1 {
    pub const CREATE: &'static str = "/channel_publication1/create";
    pub const DELETE: &'static str = "/channel_publication1/delete";
    pub const GET_MANY: &'static str = "/channel_publication1/get_many";
}
#[cfg(feature = "action_for_manual_test")]
impl ChannelPublication1 {
    pub const CREATE_: &'static str = const_format::concatcp!(
        ChannelPublication1::CREATE,
        ActionRoute::PART,
    );
    pub const DELETE_: &'static str = const_format::concatcp!(
        ChannelPublication1::DELETE,
        ActionRoute::PART,
    );
    pub const GET_MANY_: &'static str = const_format::concatcp!(
        ChannelPublication1::GET_MANY,
        ActionRoute::PART,
    );
}
pub enum ChannelPublication1Mark {
    Create,
    Delete,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
    #[cfg(feature = "action_for_manual_test")]
    Delete_,
}
impl ChannelPublication1Mark {
    pub const CREATE: &'static str = "/channel_publication1_mark/create";
    pub const DELETE: &'static str = "/channel_publication1_mark/delete";
}
#[cfg(feature = "action_for_manual_test")]
impl ChannelPublication1Mark {
    pub const CREATE_: &'static str = const_format::concatcp!(
        ChannelPublication1Mark::CREATE,
        ActionRoute::PART,
    );
    pub const DELETE_: &'static str = const_format::concatcp!(
        ChannelPublication1Mark::DELETE,
        ActionRoute::PART,
    );
}
pub enum ChannelPublication1View {
    Create,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
}
impl ChannelPublication1View {
    pub const CREATE: &'static str = "/channel_publication1_view/create";
}
#[cfg(feature = "action_for_manual_test")]
impl ChannelPublication1View {
    pub const CREATE_: &'static str = const_format::concatcp!(
        ChannelPublication1View::CREATE,
        ActionRoute::PART,
    );
}
pub enum ChannelPublication1Commentary {
    Create,
    Delete,
    #[cfg(feature = "action_for_manual_test")]
    Create_,
    #[cfg(feature = "action_for_manual_test")]
    Delete_,
}
impl ChannelPublication1Commentary {
    pub const CREATE: &'static str = "/channel_publication1_commentary/create";
    pub const DELETE: &'static str = "/channel_publication1_commentary/delete";
}
#[cfg(feature = "action_for_manual_test")]
impl ChannelPublication1Commentary {
    pub const CREATE_: &'static str = const_format::concatcp!(
        ChannelPublication1Commentary::CREATE,
        ActionRoute::PART,
    );
    pub const DELETE_: &'static str = const_format::concatcp!(
        ChannelPublication1Commentary::DELETE,
        ActionRoute::PART,
    );
}
