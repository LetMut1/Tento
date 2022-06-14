use crate::domain_layer::data_transfer_object::_in_context_for::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::header_common::HeaderCommon;
use crate::domain_layer::data_transfer_object::_in_context_for::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use super::signature_creator::SignatureCreator;

pub struct SerializationFormResolver;

impl SerializationFormResolver {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    pub fn serialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<String, ErrorAuditor> {
        match serde_json::to_vec(&HeaderCommon::new(json_access_web_token)) {
            Ok(header_common_data) => {
                let header = base64::encode_config(&header_common_data[..], base64::STANDARD);       // TODO TODO TODO base64::STANDARD - какого типа должно быть

                match serde_json::to_vec(&PayloadCommon::new(json_access_web_token)) {
                    Ok(payload_data) => {
                        let payload = base64::encode_config(&payload_data[..], base64::STANDARD);

                        let signature = SignatureCreator::create(environment_configuration_resolver, header.as_str(), payload.as_str());

                        let json_access_web_token_classic_form = header + Self::TOKEN_PARTS_SEPARATOR + payload.as_str() + Self::TOKEN_PARTS_SEPARATOR + signature.as_str();
        
                        return Ok(json_access_web_token_classic_form);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn deserialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_access_web_token_classic_form: &'a str
    ) -> Result<JsonAccessWebToken<'static>, ErrorAuditor> {
        let token_part_registry = json_access_web_token_classic_form.split::<'_, &'_ str>(Self::TOKEN_PARTS_SEPARATOR).collect::<Vec<&'_ str>>();

        if token_part_registry.len() == 3
            && SignatureCreator::is_valid(environment_configuration_resolver, token_part_registry[0], token_part_registry[1], token_part_registry[2]) {
            match base64::decode_config(token_part_registry[1].as_bytes(), base64::STANDARD) {
                Ok(data) => {
                    match serde_json::from_slice::<'_, PayloadCommon<'static>>(&data[..]) {
                        Ok(payload_common) => {
                            let (
                                json_access_web_token_id,
                                application_user_id,
                                application_user_log_in_token_device_id,
                                json_access_web_token_expiration_time
                            ) = payload_common.into_inner();
                
                            let json_access_web_token = JsonAccessWebTokenFactory::create(
                                json_access_web_token_id,
                                application_user_id,
                                application_user_log_in_token_device_id,
                                json_access_web_token_expiration_time.into_owned()
                            );
                
                            return Ok(json_access_web_token);
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        return Err(
            ErrorAuditor::new(
                BaseError::InvalidArgumentError,
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }
}