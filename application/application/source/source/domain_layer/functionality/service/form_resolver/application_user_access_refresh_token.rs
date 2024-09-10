use super::FormResolver;
use crate::{
    domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken,
    infrastructure_layer::{
        data::{
            control_type::{
                Base64,
                MessagePack,
            },
            environment_configuration::environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::{
            encoder::{
                hmac::HmacSha3_512,
                Encoder,
            },
            serializer::{
                Serialize,
                Serializer,
            },
        },
    },
};
use aggregate_error::AggregateError;
impl FormResolver<ApplicationUserAccessRefreshToken<'_>> {
    pub fn to_encrypted<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
    ) -> Result<String, AggregateError> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?;
        let encoded_data = Encoder::<HmacSha3_512>::encode(
            environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
            data.as_slice(),
        )?;
        return Ok(Encoder::<Base64>::encode(encoded_data.into_bytes().as_slice()));
    }
    pub fn is_valid<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_encrypted: &'a str,
    ) -> Result<bool, AggregateError> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?;
        let encoded_data = Encoder::<Base64>::decode(application_user_access_refresh_token_encrypted.as_bytes())?;
        return Encoder::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
            data.as_slice(),
            encoded_data.as_slice(),
        );
    }
}
