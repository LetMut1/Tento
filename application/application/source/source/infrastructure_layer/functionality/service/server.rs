use bytes::Bytes;
use http_body_util::Full;
use hyper_x::server::conn::http2::Builder;
use hyper_x::server::conn::http2::Connection;
use hyper_x::service::service_fn;
use hyper_x::service::Service as HyperService;
use hyper_x::{body::Incoming, Request, Response};
use tokio::net::TcpListener;
use std::fmt::Debug;
use std::future::Future;
use std::sync::OnceLock;
use std::time::Duration;
use hyper_x::body::Body;
use std::marker::PhantomData;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use void::Void;
use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
    OptionConverter,
    ResultConverter,
};
use hyper_util::rt::tokio::TokioExecutor;

use crate::infrastructure_layer::data::capture::Capture;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::environment_configuration::environment_configuration::EnvironmentConfiguration;

use super::spawner::Spawner;







static HTTP2_BUILDER: OnceLock<Builder<TokioExecutor>> = OnceLock::new();
pub struct Server;
impl Server {
    pub fn serve<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut http2_builder = Builder::new(TokioExecutor::new())
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
                Some(maximum_pending_accept_reset_streams_) => http2_builder.max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams_)),
                None => http2_builder.max_pending_accept_reset_streams(None),
            };
            let http2_builder_ = match HTTP2_BUILDER.get() {
                Some(http2_builder__) => http2_builder__,
                None => {
                    if let Err(_) = HTTP2_BUILDER.set(http2_builder) {
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
                    HTTP2_BUILDER.get().into_logic_value_does_not_exist(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?
                }
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
            '_a: loop {
                let tcp_stream = tcp_listener.accept().await.into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?.0;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                       return http2_builder_.serve_connection(
                            TokioIo::new(tcp_stream),
                            service_fn(
                                move |request: Request<Incoming>| -> _ {
                                    // let router___ = router__.clone();
                                    // let database_1_postgresql_connection_pool__ = database_1_postgresql_connection_pool_.clone();
                                    // let database_2_postgresql_connection_pool__ = database_2_postgresql_connection_pool_.clone();
                                    return async move {
                                        let response = Self::resolve(
                                            environment_configuration,
                                            // router___,
                                            // database_1_postgresql_connection_pool__,
                                            // database_2_postgresql_connection_pool__,
                                            request,
                                        )
                                        .await;
                                        return Ok::<_, Void>(response);
                                    };
                                }
                            ),
                        ).await
                        .into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        );
                    }
                );
            }
            return Ok(());
        };
    }
    fn resolve<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        // router: Arc<Router<ActionRoute_>>,
        // database_1_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        // database_2_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        request: Request<Incoming>,
    ) -> impl Future<Output = Response<Full<Bytes>>> + Send + Capture<&'a Void>
    // where
    //     T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    //     <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    //     <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    //     <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {


            return Response::new(Full::new(Bytes::from("Hello World!")));





        };
    }
}
// http2_builder = http2_builder
//     .tcp_nodelay(environment_configuration.application_server.tcp.nodelay)
//     .tcp_sleep_on_accept_errors(environment_configuration.application_server.tcp.sleep_on_accept_errors)
//     .tcp_keepalive_retries(environment_configuration.application_server.tcp.keepalive.retries_quantity);
// http2_builder = match environment_configuration.application_server.tcp.keepalive.duration {
//     Some(duration) => http2_builder.tcp_keepalive(Some(Duration::from_secs(duration))),
//     None => http2_builder.tcp_keepalive(None),
// };
// http2_builder = match environment_configuration.application_server.tcp.keepalive.interval_duration {
//     Some(interval_duration) => http2_builder.tcp_keepalive_interval(Some(Duration::from_secs(interval_duration))),
//     None => http2_builder.tcp_keepalive_interval(None),
// };