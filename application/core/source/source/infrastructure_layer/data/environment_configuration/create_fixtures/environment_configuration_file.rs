use crate::infrastructure_layer::data::environment_configuration::PostgresqlInner_;
#[derive(serde::Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub resource: Resource,
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