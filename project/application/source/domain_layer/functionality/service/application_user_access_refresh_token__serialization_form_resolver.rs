use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::encoder::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;
use crate::infrastructure_layer::functionality::service::encoder::Hmac;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;

pub struct ApplicationUserAccessRefreshToken_SerializationFormResolver;

impl ApplicationUserAccessRefreshToken_SerializationFormResolver {
    pub fn encode<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<String, ErrorAuditor> {
        let data = match Serializer::<MessagePack>::serialize(application_user_access_refresh_token) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut hmac_encoded_data: Vec<u8> = vec![];

        Encoder_::<Hmac>::encode(
            environment_configuration.get_security_auart_encoding_private_key().as_bytes(),
            data.as_slice(),
            hmac_encoded_data.as_mut_slice()
        );

        let application_user_access_refresh_token_deserialized_form = Encoder_::<Base64>::encode(hmac_encoded_data.as_slice());

        return Ok(application_user_access_refresh_token_deserialized_form);
    }

    pub fn is_valid<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_deserialized_form: &'a str
    ) -> Result<bool, ErrorAuditor> {
        let application_user_access_refresh_token_deserialized_form_ = match Self::encode(
            environment_configuration,
            application_user_access_refresh_token
        ) {
            Ok(application_user_access_refresh_token_deserialized_form__) => application_user_access_refresh_token_deserialized_form__,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(
            application_user_access_refresh_token_deserialized_form_.as_str() == application_user_access_refresh_token_deserialized_form
        );
    }
}