mod environment_configuration_file;
pub use self::environment_configuration_file::EnvironmentConfigurationFile;
use super::Postgresql;
pub struct CreateFixtures {
    pub resource: Resource,
}
pub struct Resource {
    pub postgresql: Postgresql,
}
