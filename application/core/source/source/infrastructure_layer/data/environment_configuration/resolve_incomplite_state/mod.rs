mod environment_configuration_file;
pub use self::environment_configuration_file::EnvironmentConfigurationFile;
use super::PostgresqlInner;
pub struct ResolveIncompliteState {
    pub system: System,
    #[cfg(feature = "logging_to_file")]
    pub logging: Logging,
    pub resource: Resource,
}
pub struct System {
    pub tokio: Tokio,
}
pub struct Tokio {
    pub worker_threads_quantity: u16,
    pub worker_thread_stack_size: usize,
}
#[cfg(feature = "logging_to_file")]
pub struct Logging {
    pub directory_path: String,
    pub file_name_prefix: String,
}
pub struct Resource {
    pub postgresql: Postgresql,
}
pub struct Postgresql {
    pub database_1: PostgresqlInner,
    pub database_2: PostgresqlInner,
    pub database_3: PostgresqlInner,
}
