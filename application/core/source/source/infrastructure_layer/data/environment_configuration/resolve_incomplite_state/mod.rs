mod environment_configuration_file;
pub use self::environment_configuration_file::EnvironmentConfigurationFile;
use super::PostgresqlInner;
pub struct ResolveIncompliteState {
    pub tokio_runtime: TokioRuntime,
    #[cfg(feature = "logging_to_file")]
    pub logging: Logging,
    pub resource: Resource,
}
pub struct TokioRuntime {
    pub worker_threads_quantity: usize,
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
