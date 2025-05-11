mod environment_configuration_file;
pub use self::environment_configuration_file::EnvironmentConfigurationFile;
use {
    super::PostgresqlInner,
    std::net::SocketAddr,
};
pub struct RunServer {
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    #[cfg(feature = "logging_to_file")]
    pub logging: Logging,
    pub resource: Resource,
    pub encryption: Encryption,
}
pub struct TokioRuntime {
    pub worker_threads_quantity: usize,
    pub worker_thread_stack_size: usize,
}
pub struct ApplicationServer {
    pub tcp: Tcp,
    pub http: Http,
}
pub struct Tcp {
    pub socket_address: SocketAddr,
    pub nodelay: bool,
    pub sleep_on_accept_errors: bool,
    pub keepalive: TcpKeepalive,
}
pub struct TcpKeepalive {
    pub duration: Option<u64>,
    pub interval_duration: Option<u64>,
    pub retries_quantity: Option<u32>,
}
pub struct Http {
    pub adaptive_window: bool,
    pub connection_window_size: u32,
    pub stream_window_size: u32,
    pub maximum_frame_size: u32,
    pub maximum_sending_buffer_size: u32,
    pub enable_connect_protocol: bool,
    pub maximum_header_list_size: u32,
    pub maximum_pending_accept_reset_streams: Option<usize>,
    pub keepalive: Option<HttpKeepalive>,
    pub tls: Option<Tls>,
}
pub struct HttpKeepalive {
    pub interval_duration: u64,
    pub timeout_duration: u64,
}
pub struct Tls {
    pub certificate_crt_file_path: String,
    pub certificate_key_file_path: String,
}
#[cfg(feature = "logging_to_file")]
pub struct Logging {
    pub directory_path: String,
    pub file_name_prefix: String,
}
pub struct Resource {
    pub postgresql: Postgresql,
    pub email_server: EmailServer,
}
pub struct Postgresql {
    pub database_1: PostgresqlInner,
    pub database_2: PostgresqlInner,
    pub database_3: PostgresqlInner,
    pub database_4: PostgresqlInner,
}
pub struct EmailServer {
    pub socket_address: SocketAddr,
}
pub struct Encryption {
    pub private_key: PrivateKey,
}
pub struct PrivateKey {
    pub user_access_token: String,
    pub user_access_refresh_token: String,
    pub channel_token: String,
    pub channel_subscription_token: String,
    pub channel_publication1_token: String,
}
