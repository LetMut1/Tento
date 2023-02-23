use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::base64;
use extern_crate::crypto::hmac::Hmac;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use extern_crate::hex;
use extern_crate::rmp_serde;

pub struct ApplicationUserAccessToken_SerializationFormResolver;

impl ApplicationUserAccessToken_SerializationFormResolver {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    pub fn serialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfiguration,
        application_user_access_token: &'a ApplicationUserAccessToken<'_>
    ) -> Result<String, ErrorAuditor> {
        let mut data: Vec<u8> = vec![];
        if let Err(error) = rmp_serde::encode::write(&mut data, application_user_access_token) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        let application_user_access_token_serialized = base64::encode_config(data.as_slice(), base64::STANDARD);  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?

        let application_user_access_token_signature = ApplicationUserAccessToken_Encoder::create(
            environment_configuration_resolver, application_user_access_token_serialized.as_str()
        );

        let application_user_access_token_deserialized_form = application_user_access_token_serialized + Self::TOKEN_PARTS_SEPARATOR + application_user_access_token_signature.as_str();

        return Ok(application_user_access_token_deserialized_form);
    }

    pub fn deserialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfiguration,
        application_user_access_token_deserialized_form: &'a str
    ) -> Result<ArgumentResult<ApplicationUserAccessToken<'static>>, ErrorAuditor> {
        let token_part_registry = application_user_access_token_deserialized_form
            .split::<'_, &'_ str>(Self::TOKEN_PARTS_SEPARATOR)
            .collect::<Vec<&'_ str>>();                                                         // TODO проверить, правильно ли вот тут вообще

        if token_part_registry.len() != 2
            || !ApplicationUserAccessToken_Encoder::is_valid(environment_configuration_resolver, token_part_registry[0], token_part_registry[1]) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserAccessToken_DeserializedForm });
        }

        let data = match base64::decode_config(token_part_registry[0].as_bytes(), base64::STANDARD) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_access_token = match rmp_serde::from_read_ref::<'_, [u8], ApplicationUserAccessToken<'static>>(data.as_slice()) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(ArgumentResult::Ok { subject: application_user_access_token });
    }
}

struct ApplicationUserAccessToken_Encoder;

impl ApplicationUserAccessToken_Encoder {
    fn create<'a>(
        environment_configuration_resolver: &'a EnvironmentConfiguration,
        application_user_access_token_serialized: &'a str
    ) -> String {
        let mut hmac = Hmac::new(
            Sha512::new(),
            environment_configuration_resolver.get_security_auat_signature_encoding_private_key().as_bytes()
        );
        hmac.input(application_user_access_token_serialized.as_bytes());

        return hex::encode(hmac.result().code());   // TODO TIme attack
    }

    fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfiguration,
        application_user_access_token_serialized: &'a str,
        application_user_access_token_signature: &'a str
    ) -> bool {
        return Self::create(environment_configuration_resolver, application_user_access_token_serialized).as_bytes()
            == application_user_access_token_signature.as_bytes();
    }
}