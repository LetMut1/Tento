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
            creator::{
                router::Router,
                Creator,
            },
            logger::Logger,
            spawner::{
                tokio_non_blocking_task::TokioNonBlockingTask,
                Spawner,
            },
        },
    },
    presentation_layer::{
        data::action_route::{
            ActionRoute_,
            ApplicationUser__Authorization_,
            ChannelSubscription__Base_,
            Channel__Base_,
        },
        functionality::action::{
            route_not_found::RouteNotFound,
            Action,
            Inner as ActionInner,
        },
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
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
                    router: Creator::<Router>::create()?,
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
                            result = tcp_accepting_future => {
                                let result_ = match result {
                                    Result::Ok(result__) => result__,
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
                                #[cfg(feature = "manual_testing")]
                                let (tcp_stream, socket_address) = result_;
                                #[cfg(not(feature = "manual_testing"))]
                                let (tcp_stream, _) = result_;
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
                                if http1_socket_address.port() == socket_address.port() {
                                    Self::spawn_connection_serving(
                                        http1_builder.serve_connection(
                                            TokioIo::new(tcp_stream),
                                            service_fn,
                                        ),
                                    );
                                } else {
                                    Self::spawn_connection_serving(
                                        http2_builder.serve_connection(
                                            TokioIo::new(tcp_stream),
                                            service_fn,
                                        ),
                                    );
                                };
                                #[cfg(not(feature = "manual_testing"))]
                                Self::spawn_connection_serving(
                                    http2_builder.serve_connection(
                                        TokioIo::new(tcp_stream),
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
    fn process_request<T>(request: Request, environment_configuration: &'static EnvironmentConfiguration, cloned: Arc<Cloned<T>>) -> impl Future<Output = Response> + Send
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
    router: Router,
    database_1_postgresql_connection_pool: Pool<PostgresConnectionManager<T>>,
    database_2_postgresql_connection_pool: Pool<PostgresConnectionManager<T>>,
}
