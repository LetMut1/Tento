use super::form_resolver::FormResolver;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::encoder::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;
use crate::infrastructure_layer::functionality::service::encoder::Hmac;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;

impl FormResolver<ApplicationUserAccessRefreshToken<'_>> {
    pub fn to_encrypted<'a>(application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>) -> Result<ApplicationUserAccessRefreshTokenEncrypted, ErrorAuditor> {
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

        let mut hmac_encoded_data: Vec<u8> = vec![];

        Encoder_::<Hmac>::encode(
            ENVIRONMENT_CONFIGURATION.encryption.private_key.application_user_access_refresh_token.0.as_bytes(),
            data.as_slice(),
            hmac_encoded_data.as_mut_slice(),
        );

        let application_user_access_refresh_token_encrypted = Encoder_::<Base64>::encode(hmac_encoded_data.as_slice());

        return Ok(ApplicationUserAccessRefreshTokenEncrypted::new(application_user_access_refresh_token_encrypted));
    }

    pub fn is_valid<'a>(
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_encrypted: &'a ApplicationUserAccessRefreshTokenEncrypted,
    ) -> Result<bool, ErrorAuditor> {
        let application_user_access_refresh_token_encrypted_ = match Self::to_encrypted(application_user_access_refresh_token) {
            Ok(application_user_access_refresh_token_encrypted__) => application_user_access_refresh_token_encrypted__,
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

        return Ok(application_user_access_refresh_token_encrypted_.get() == application_user_access_refresh_token_encrypted.get());
    }
}
