use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;

pub trait SignatureCreatorTrait {
    fn create<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        header: &'a str,
        payload: &'a str
    ) -> String;

    fn is_valid<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> bool;
}