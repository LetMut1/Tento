use self::sealed::Sealed;

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
    pub encryption: Encryption<T>,
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

pub struct String_(pub String);

pub struct StringLiteral(pub &'static str);

mod sealed {
    use super::StringLiteral;
    use super::String_;

    pub trait Sealed {}

    impl Sealed for StringLiteral {}

    impl Sealed for String_ {}
}

pub(crate) mod environment_configuration_file {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct EnvironmentConfigurationFile {
        pub application_server: ApplicationServer,
        pub resource: Resource,
        pub encryption: Encryption,
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
        pub certificate_crt_path: Value<String>,
        pub certificate_key_path: Value<String>,
    }

    #[derive(Deserialize)]
    pub struct Resource {
        pub postgresql: Postgresql,
        pub redis: Redis,
        pub email_server: EmailServer,
    }

    #[derive(Deserialize)]
    pub struct Postgresql {
        pub database_1_url: Value<String>,
        pub database_2_url: Value<String>,
    }

    #[derive(Deserialize)]
    pub struct Redis {
        pub database_1_url: Value<String>,
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
}