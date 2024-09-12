use serde::Deserialize;
#[derive(Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    pub logging: Logging,
    pub resource: Resource,
    pub encryption: Encryption,
}
#[derive(Deserialize)]
pub struct TokioRuntime {
    pub maximum_blocking_threads_quantity: Value<usize>,
    pub worker_threads_quantity: Value<usize>,
    pub worker_thread_stack_size: Value<usize>,
}
#[derive(Deserialize)]
pub struct ApplicationServer {
    pub tcp: Tcp,
    pub http: Http,
}
#[derive(Deserialize)]
pub struct Tcp {
    pub socket_address: Value<String>,
    pub nodelay: Value<bool>,
    pub sleep_on_accept_errors: Value<bool>,
    pub keepalive: TcpKeepalive,
}
#[derive(Deserialize)]
pub struct TcpKeepalive {
    pub duration: ValueExist<u64>,
    pub interval_duration: ValueExist<u64>,
    pub retries_quantity: ValueExist<u32>,
}
#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct HttpKeepalive {
    pub is_exist: bool,
    pub interval_duration: Value<u64>,
    pub timeout_duration: Value<u64>,
}
#[derive(Deserialize)]
pub struct Tls {
    pub is_exist: bool,
    pub certificate_crt_file_path: Value<String>,
    pub certificate_key_file_path: Value<String>,
}
#[derive(Deserialize)]
pub struct Logging {
    pub directory_path: Value<String>,
    pub file_name_prefix: Value<String>,
}
#[derive(Deserialize)]
pub struct Resource {
    pub postgresql: Postgresql,
    pub email_server: EmailServer,
}
#[derive(Deserialize)]
pub struct Postgresql {
    pub database_1_url: Value<String>,
    pub database_2_url: Value<String>,
}
#[derive(Deserialize)]
pub struct EmailServer {
    pub socket_address: Value<String>,
}
#[derive(Deserialize)]
pub struct Encryption {
    pub private_key: PrivateKey,
}
#[derive(Deserialize)]
pub struct PrivateKey {
    pub application_user_access_token: Value<String>,
    pub application_user_access_refresh_token: Value<String>,
}
#[derive(Deserialize)]
pub struct Value<T> {
    pub value: T,
}
#[derive(Deserialize)]
pub struct ValueExist<T> {
    pub value: T,
    pub is_exist: bool,
}
