use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::message_pack_serializer::MessagePack;
use crate::infrastructure_layer::functionality::service::message_pack_serializer::Serialize;
use crate::infrastructure_layer::functionality::service::message_pack_serializer::Serializer;
use extern_crate::crypto::hmac::Hmac;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use extern_crate::hex;

pub struct ApplicationUserAccessRefreshToken_SerializationFormResolver;

impl ApplicationUserAccessRefreshToken_SerializationFormResolver {
    pub fn encode<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<String, ErrorAuditor> {
        let data = match Serializer::serialize(application_user_access_refresh_token) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut hmac = Hmac::new(
            Sha512::new(),
            environment_configuration.get_security_auart_encoding_private_key().as_bytes()
        );
        hmac.input(data.as_slice());

        let application_user_access_refresh_token_deserialized_form = hex::encode(hmac.result().code());     // TODO  TODO TODO time attac// TODO TODO TODO TODO TODO Валидно ли кодирует ХЕКС, если это Байты МессаджПака?

        return Ok(application_user_access_refresh_token_deserialized_form);
    }

    pub fn is_valid<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_deserialized_form: &'a str
    ) -> Result<bool, ErrorAuditor> {
        let application_user_access_refresh_token_deserialized_form_ = match Self::encode(
            environment_configuration, application_user_access_refresh_token
        ) {
            Ok(application_user_access_refresh_token_deserialized_form__) => application_user_access_refresh_token_deserialized_form__,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(
            application_user_access_refresh_token_deserialized_form_.as_bytes() == application_user_access_refresh_token_deserialized_form.as_bytes()
        );
    }
}