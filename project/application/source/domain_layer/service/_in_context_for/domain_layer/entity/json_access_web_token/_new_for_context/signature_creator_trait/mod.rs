use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub trait SignatureCreatorTrait {
    fn create<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        header: &'a str,
        payload: &'a str
    ) -> String;

    fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> bool;
}