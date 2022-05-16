use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::error::Error;

pub trait EncoderTrait {
    type Error: Error;

    fn encode<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<String, Self::Error>;

    fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>,
        json_refresh_web_token_hash: &'a str
    ) -> Result<bool, Self::Error>;
}