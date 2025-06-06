use crate::infrastructure_layer::data::environment_configuration::{
    PostgresqlInner_,
    Value,
};
#[derive(serde::Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub system: System,
    #[cfg(feature = "logging_to_file")]
    pub logging: Logging,
    pub resource: Resource,
}
#[derive(serde::Deserialize)]
pub struct System {
    pub tokio: Tokio,
}
#[derive(serde::Deserialize)]
pub struct Tokio {
    pub worker_threads_quantity: Value<u16>,
    pub affinited_cores: Value<Vec<u8>>,
    pub worker_thread_stack_size: Value<usize>,
}
#[cfg(feature = "logging_to_file")]
#[derive(serde::Deserialize)]
pub struct Logging {
    pub directory_path: Value<String>,
    pub file_name_prefix: Value<String>,
}
#[derive(serde::Deserialize)]
pub struct Resource {
    pub postgresql: Postgresql,
}
#[derive(serde::Deserialize)]
pub struct Postgresql {
    pub database_1: PostgresqlInner_,
    pub database_2: PostgresqlInner_,
    pub database_3: PostgresqlInner_,
}
