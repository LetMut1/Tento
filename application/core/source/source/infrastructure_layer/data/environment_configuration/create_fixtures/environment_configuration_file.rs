use crate::infrastructure_layer::data::environment_configuration::Postgresql_;
#[derive(serde::Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub resource: Resource,
}
#[derive(serde::Deserialize)]
pub struct Resource {
    pub postgresql: Postgresql_,
}
