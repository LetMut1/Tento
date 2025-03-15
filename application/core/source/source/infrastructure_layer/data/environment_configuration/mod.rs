pub mod create_fixtures;
pub mod run_server;
pub mod resolve_incomplite_state;
use tokio_postgres::config::Config;
pub struct EnvironmentConfiguration<S> {
    pub subject: S,
}
pub struct PostgresqlInner {
    pub configuration: Config,
    pub maximum_connection_pool_size: usize,
    pub connection_pool_waiting_timeout_duration: u64,
}
#[derive(serde::Deserialize)]
pub struct PostgresqlInner_ {
    pub url: Value<String>,
    pub maximum_connection_pool_size: Value<usize>,
    pub connection_pool_waiting_timeout_duration: Value<u64>,
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
