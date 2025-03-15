mod environment_configuration_file;
pub use self::environment_configuration_file::EnvironmentConfigurationFile;
use super::PostgresqlInner;
pub struct CreateFixtures {
    pub resource: Resource,
}
pub struct Resource {
    pub postgresql: Postgresql,
}
pub struct Postgresql {
    pub database_1: PostgresqlInner,
    pub database_2: PostgresqlInner,
    pub database_3: PostgresqlInner,
}
