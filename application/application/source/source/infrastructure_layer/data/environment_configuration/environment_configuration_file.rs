#[derive(serde::Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    pub logging: Logging,
    pub resource: Resource,
    pub encryption: Encryption,
}
#[derive(serde::Deserialize)]
pub struct TokioRuntime {
    pub maximum_blocking_threads_quantity: Value<usize>,
    pub worker_threads_quantity: Value<usize>,
    pub worker_thread_stack_size: Value<usize>,
}
#[derive(serde::Deserialize)]
pub struct ApplicationServer {
    pub tcp: Tcp,
    pub http: Http,
}
#[derive(serde::Deserialize)]
pub struct Tcp {
    pub socket_address: Value<String>,
    pub nodelay: Value<bool>,
    pub sleep_on_accept_errors: Value<bool>,
    pub keepalive: TcpKeepalive,
}
#[derive(serde::Deserialize)]
pub struct TcpKeepalive {
    pub duration: ValueExist<u64>,
    pub interval_duration: ValueExist<u64>,
    pub retries_quantity: ValueExist<u32>,
}
#[derive(serde::Deserialize)]
pub struct Http {
    pub adaptive_window: Value<bool>,
    pub connection_window_size: Value<u32>,
    pub stream_window_size: Value<u32>,
    pub maximum_frame_size: Value<u32>,
    pub maximum_sending_buffer_size: Value<u32>,
    pub enable_connect_protocol: Value<bool>,
    pub maximum_header_list_size: Value<u32>,
    pub maximum_pending_accept_reset_streams: ValueExist<usize>,
    pub keepalive: HttpKeepalive,
    pub tls: Tls,
}
#[derive(serde::Deserialize)]
pub struct HttpKeepalive {
    pub is_exist: bool,
    pub interval_duration: Value<u64>,
    pub timeout_duration: Value<u64>,
}
#[derive(serde::Deserialize)]
pub struct Tls {
    pub is_exist: bool,
    pub certificate_crt_file_path: Value<String>,
    pub certificate_key_file_path: Value<String>,
}
#[derive(serde::Deserialize)]
pub struct Logging {
    pub directory_path: Value<String>,
    pub file_name_prefix: Value<String>,
}
#[derive(serde::Deserialize)]
pub struct Resource {
    pub postgresql: Postgresql,
    pub email_server: EmailServer,
}
#[derive(serde::Deserialize)]
pub struct Postgresql {
    pub database_1_url: Value<String>,
    pub database_2_url: Value<String>,
}
#[derive(serde::Deserialize)]
pub struct EmailServer {
    pub socket_address: Value<String>,
}
#[derive(serde::Deserialize)]
pub struct Encryption {
    pub private_key: PrivateKey,
}
#[derive(serde::Deserialize)]
pub struct PrivateKey {
    pub user_access_token: Value<String>,
    pub user_access_refresh_token: Value<String>,
}
#[derive(serde::Deserialize)]
pub struct Value<T> {
    pub value: T,
}
#[derive(serde::Deserialize)]
pub struct ValueExist<T> {
    pub value: T,
    pub is_exist: bool,
}
