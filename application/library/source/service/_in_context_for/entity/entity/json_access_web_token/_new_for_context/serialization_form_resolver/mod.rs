use crate::data_transfer_object::_in_context_for::entity::entity::json_access_web_token::core::header::header::_new_for_context::common::Common as HeaderCommon;
use crate::data_transfer_object::_in_context_for::entity::entity::json_access_web_token::core::payload::_new_fro_context::common::Common as PayloadCommon;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error::core::invalid_argument_error::InvalidArgumentError;
use crate::utility::_in_context_for::entity::entity::json_access_web_token::core::_new_for_context::signature_creator::SignatureCreator;
use serde_json;

pub struct SerializationFormResolver;

impl SerializationFormResolver {
    const SEPARATOR: &'static str = ".";

    pub fn serialize<'outer_a>(json_access_web_token: &'outer_a JsonAccessWebToken<'_>) -> String {
        let header_and_payload: String = 
        base64::encode(serde_json::to_string(&HeaderCommon::new(json_access_web_token)).unwrap().as_bytes()) 
        + Self::SEPARATOR 
        + base64::encode(serde_json::to_string(&PayloadCommon::new(json_access_web_token)).unwrap().as_bytes()).as_str();
        
        let signature: String = SignatureCreator::create(&header_and_payload);

        return header_and_payload + Self::SEPARATOR + signature.as_str();
    }

    pub fn deserialize<'outer_a, 'vague>(classic_form: &'outer_a str) -> Result<JsonAccessWebToken<'vague>, InvalidArgumentError> {
        let classic_form_parts: Vec<&'_ str> = classic_form.split::<'_, &'_ str>(Self::SEPARATOR).collect::<Vec<&'_ str>>();

        if classic_form_parts.len() == 3 {
            if SignatureCreator::is_valid(
                (String::new() + classic_form_parts[0] + Self::SEPARATOR + classic_form_parts[1]).as_str(), classic_form_parts[2]
            ) 
            {
                return Ok(
                    JsonAccessWebToken::new_from_payload_common(
                        serde_json::from_slice::<'_, PayloadCommon>(&base64::decode(classic_form_parts[1].as_bytes()).unwrap()).unwrap()  // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
                    )?
                );
            }
        }

        return Err(InvalidArgumentError);
    }
} 