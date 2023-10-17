use super::FormResolver;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::functionality::service::encoder::base64::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::functionality::service::encoder::hmac::Hmac_Sha3_512;
use crate::infrastructure_layer::functionality::service::serializer::message_pack::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;

impl FormResolver<ApplicationUserAccessRefreshToken<'_>> {
    pub fn to_encrypted<'a>(application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>) -> Result<ApplicationUserAccessRefreshTokenEncrypted, ErrorAuditor_> {
        let data = match Serializer::<MessagePack>::serialize(application_user_access_refresh_token) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let encoded_data = match Encoder::<Hmac_Sha3_512>::encode(
            ENVIRONMENT_CONFIGURATION.encryption.private_key.application_user_access_refresh_token.0.as_bytes(),
            data.as_slice(),
        )
        {
            Ok(encoded_data_) => encoded_data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_refresh_token_encrypted = Encoder::<Base64>::encode(
            encoded_data.into_bytes().as_slice()
        );

        return Ok(ApplicationUserAccessRefreshTokenEncrypted(application_user_access_refresh_token_encrypted));
    }

    pub fn is_valid<'a>(
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_encrypted: &'a ApplicationUserAccessRefreshTokenEncrypted,
    ) -> Result<bool, ErrorAuditor_> {
        let data = match Serializer::<MessagePack>::serialize(application_user_access_refresh_token) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let encoded_data = match Encoder::<Base64>::decode(application_user_access_refresh_token_encrypted.0.as_bytes()) {
            Ok(encoded_data_) => encoded_data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let is_valid = match Encoder::<Hmac_Sha3_512>::is_valid(
            ENVIRONMENT_CONFIGURATION.encryption.private_key.application_user_access_refresh_token.0.as_bytes(),
            data.as_slice(),
            encoded_data.as_slice(),
        )
        {
            Ok(is_valid_) => is_valid_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(is_valid);
    }
}
