use super::FormResolver;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::encoder::base64::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::functionality::service::encoder::hmac::HmacSha3512;
use crate::infrastructure_layer::functionality::service::serializer::message_pack::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::data::error::Error;

impl FormResolver<ApplicationUserAccessRefreshToken<'_>> {
    pub fn to_encrypted<'a>(environment_configuration: &'a EnvironmentConfiguration, application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>) -> Result<ApplicationUserAccessRefreshTokenEncrypted, Auditor<Error>> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?;

        let encoded_data = Encoder::<HmacSha3512>::encode(
            environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
            data.as_slice(),
        )?;

        let application_user_access_refresh_token_encrypted = Encoder::<Base64>::encode(
            encoded_data.into_bytes().as_slice()
        );

        return Ok(ApplicationUserAccessRefreshTokenEncrypted(application_user_access_refresh_token_encrypted));
    }

    pub fn is_valid<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_encrypted: &'a ApplicationUserAccessRefreshTokenEncrypted,
    ) -> Result<bool, Auditor<Error>> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?;

        let encoded_data = Encoder::<Base64>::decode(application_user_access_refresh_token_encrypted.0.as_bytes())?;

        return Ok(
            Encoder::<HmacSha3512>::is_valid(
                environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
                data.as_slice(),
                encoded_data.as_slice(),
            )?
        );
    }
}
