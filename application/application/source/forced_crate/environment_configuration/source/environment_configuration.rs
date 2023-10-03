use super::sealed::Sealed;

pub enum Environment {
    Production,
    Development,
    LocalDevelopment,
}

pub struct EnvironmentConfiguration<T>
where
    T: Sealed,
{
    pub environment: Environment,
    pub application_server: ApplicationServer<T>,
    pub resource: Resource<T>,
    pub encryption: Encryption<T>,  // TODO Заменить на Security слово.
}

pub struct ApplicationServer<T>
where
    T: Sealed,
{
    pub tcp: Tcp<T>,
    pub http: Http<T>,
}

pub struct Tcp<T>
where
    T: Sealed,
{
    pub socket_address: T,
    pub nodelay: bool,
    pub sleep_on_accept_errors: bool,
    pub keepalive: TcpKeepalive,
}

pub struct TcpKeepalive {
    pub duration: Option<u64>,
    pub interval_duration: Option<u64>,
    pub retries_quantity: Option<u32>,
}

pub struct Http<T>
where
    T: Sealed,
{
    pub adaptive_window: bool,
    pub connection_window_size: u32,
    pub stream_window_size: u32,
    pub maximum_frame_size: u32,
    pub maximum_sending_buffer_size: u32,
    pub enable_connect_protocol: bool,
    pub maximum_header_list_size: u32,
    pub maximum_pending_accept_reset_streams: Option<usize>,
    pub keepalive: Option<HttpKeepalive>,
    pub tls: Option<Tls<T>>,
}

pub struct HttpKeepalive {
    pub interval_duration: u64,
    pub timeout_duration: u64,
}

pub struct Tls<T>
where
    T: Sealed,
{
    pub certificate_crt_path: T,
    pub certificate_key_path: T,
}

pub struct Resource<T>
where
    T: Sealed,
{
    pub postgresql: Postgresql<T>,
    pub redis: Redis<T>,
    pub email_server: EmailServer<T>,
}

pub struct Postgresql<T>
where
    T: Sealed,
{
    pub database_1_url: T,
    pub database_2_url: T,
}

pub struct Redis<T>
where
    T: Sealed,
{
    pub database_1_url: T,
}

pub struct EmailServer<T>
where
    T: Sealed,
{
    pub socket_address: T,
}

pub struct Encryption<T>
where
    T: Sealed,
{
    pub private_key: PrivateKey<T>,
}

pub struct PrivateKey<T>
where
    T: Sealed,
{
    pub application_user_access_token: T,
    pub application_user_access_refresh_token: T,
}