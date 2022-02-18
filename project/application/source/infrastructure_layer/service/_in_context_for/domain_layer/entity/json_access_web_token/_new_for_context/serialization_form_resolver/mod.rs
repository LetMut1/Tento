
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::signature_creator_trait::SignatureCreatorTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::header_common::HeaderCommon;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::signature_creator::SignatureCreator;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;

pub struct SerializationFormResolver;

impl SerializationFormResolverTrait for SerializationFormResolver {
    type Error = BaseError;

    fn serialize<'a>(
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<String, Self::Error> {
        let header = base64::encode_config(serde_json::to_vec(&HeaderCommon::new(json_access_web_token))?, base64::URL_SAFE);

        let payload = base64::encode_config(serde_json::to_vec(&PayloadCommon::new(json_access_web_token))?, base64::URL_SAFE);
        
        let signature = SignatureCreator::create(header.as_str(), payload.as_str())?;

        let json_access_web_token_classic_form = header + Self::TOKEN_PARTS_SEPARATOR + payload.as_str() + Self::TOKEN_PARTS_SEPARATOR + signature.as_str();

        return Ok(json_access_web_token_classic_form);
    }

    fn deserialize<'a>(
        classic_form: &'a str
    ) -> Result<JsonAccessWebToken<'static>, Self::Error> {
        let token_part_registry = classic_form.split::<'_, &'_ str>(Self::TOKEN_PARTS_SEPARATOR).collect::<Vec<&'_ str>>();

        if token_part_registry.len() == 3 && SignatureCreator::is_valid(token_part_registry[0], token_part_registry[1], token_part_registry[2])? {
            let payload_common = serde_json::from_slice::<'_, PayloadCommon<'static>>(
                &base64::decode_config(token_part_registry[1].as_bytes(), base64::URL_SAFE)?[..]
            )?;

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

        return Err(BaseError::InvalidArgumentError);
    }
} 