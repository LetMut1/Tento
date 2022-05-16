use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::error::Error;

pub trait SerializationFormResolverTrait {
    type Error: Error;

    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    fn serialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<String, Self::Error>;

    fn deserialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_access_web_token_classic_form: &'a str
    ) -> Result<JsonAccessWebToken<'static>, Self::Error>;
}