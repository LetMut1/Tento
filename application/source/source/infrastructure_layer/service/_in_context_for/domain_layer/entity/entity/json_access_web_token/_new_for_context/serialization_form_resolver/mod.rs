use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::utility::_in_context_for::entity::entity::json_access_web_token::_core::_new_for_context::signature_creator::SignatureCreator;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::entity::entity::json_access_web_token::_core::header::header::_new_for_context::common::Common as HeaderCommon;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::entity::entity::json_access_web_token::_core::payload::payload::_new_fro_context::common::Common as PayloadCommon;

pub struct SerializationFormResolver;

impl SerializationFormResolverTrait for SerializationFormResolver {
    fn serialize<'outer_a>(json_access_web_token: &'outer_a JsonAccessWebToken<'_>) -> Result<String, BaseError> {
        let header_and_payload: String = 
        base64::encode(serde_json::to_string(&HeaderCommon::new(json_access_web_token))?.as_bytes()) 
        + Self::TOKEN_PARTS_SEPARATOR 
        + base64::encode(serde_json::to_string(&PayloadCommon::new(json_access_web_token))?.as_bytes()).as_str();
        
        let signature: String = SignatureCreator::create(&header_and_payload)?;

        return Ok(header_and_payload + Self::TOKEN_PARTS_SEPARATOR + signature.as_str());
    }

    fn deserialize<'outer_a, 'vague>(classic_form: &'outer_a str) -> Result<JsonAccessWebToken<'vague>, BaseError> {
        let classic_form_parts: Vec<&'_ str> = classic_form.split::<'_, &'_ str>(Self::TOKEN_PARTS_SEPARATOR).collect::<Vec<&'_ str>>();

        if classic_form_parts.len() == 3 
        && SignatureCreator::is_valid(
            (String::new() + classic_form_parts[0] + Self::TOKEN_PARTS_SEPARATOR + classic_form_parts[1]).as_str(), classic_form_parts[2]
        )?
        {
            return Ok(
                JsonAccessWebToken::new_from_common(
                    serde_json::from_slice::<'_, PayloadCommon>(&base64::decode(classic_form_parts[1].as_bytes())?)?
                )?
            );
        }

        return Err(BaseError::InvalidArgumentError);
    }
} 