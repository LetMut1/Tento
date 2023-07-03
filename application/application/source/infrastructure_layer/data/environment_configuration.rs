use extern_crate::serde::Deserialize;

pub struct EnvironmentConfiguration {
    pub environment: Environment,
    pub environment_file_configuration: EnvironmentFileConfiguration,
}

pub enum Environment {
    Production,
    Development,
    LocalDevelopment,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct EnvironmentFileConfiguration {
    pub application: Application,
    pub resource: Resource,
    pub encryption: Encryption,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Application {
    pub tcp: Tcp,
    pub http: Http,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Tcp {
    pub socket_address: String_,
    pub nodelay: Bool,
    pub sleep_on_accept_errors: Bool,
    pub keepalive_seconds: U64Active,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Http {
    pub adaptive_window: Bool,
    pub connection_window_size: U32,
    pub stream_window_size: U32,
    pub maximum_frame_size: U32,
    pub maximum_sending_buffer_size: U32,
    pub http2_only: Bool,
    pub keep_alive: KeepAlive,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct KeepAlive {
    pub is_active: bool,
    pub interval_seconds: U64,
    pub timeout_seconds: U64,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Resource {
    pub postgresql: Postgresql,
    pub redis: Redis,
    pub email_server: EmailServer,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Postgresql {
    pub database_1_url: String_,
    pub database_2_url: String_,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Redis {
    pub database_1_url: String_,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct EmailServer {
    pub socket_address: String_,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Encryption {
    pub private_key: PrivateKey,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct PrivateKey {
    pub application_user_access_token: String_,
    pub application_user_access_refresh_token: String_,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct String_ {
    pub value: String,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Bool {
    pub value: bool,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct U64Active {
    pub value: u64,
    pub is_active: bool,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct U64 {
    pub value: u64,
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct U32 {
    pub value: u32,
}
