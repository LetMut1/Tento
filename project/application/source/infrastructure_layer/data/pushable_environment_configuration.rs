use super::environment_configuration::Environment;
use std::net::SocketAddr;

pub struct PushableEnvironmentConfiguration {
    pub environment: Environment,
    pub encryption: Encryption,
    pub resource: Resource,
}

pub struct Encryption {
    pub private_key: PrivateKey,
}

pub struct PrivateKey {
    pub application_user_access_token: String,
    pub application_user_access_refresh_token: String,
}

pub struct Resource {
    pub email_server: EmailServer,
}

pub struct EmailServer {
    pub socket_address: SocketAddr,
}
