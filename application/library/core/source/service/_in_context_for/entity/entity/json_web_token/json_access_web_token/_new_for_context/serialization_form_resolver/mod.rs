use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::header::header::_new_for_context::common::Common as HeaderCommon;
use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common as PayloadCommon;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::_new_for_context::signature_creator::SignatureCreator;
use serde_json;

pub struct SerializationFormResolver;

impl<'outer, 'vague> SerializationFormResolver {
    const LINE_SEPARATOR: &'static str = ".";

    pub fn serialize(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> String {
        let header_and_payload: String = 
        base64::encode(serde_json::to_string(&HeaderCommon::new(json_access_web_token)).unwrap().as_bytes()) 
        + Self::LINE_SEPARATOR 
        + base64::encode(serde_json::to_string(&PayloadCommon::new(json_access_web_token)).unwrap().as_bytes()).as_str();
        
        let signature: String = SignatureCreator::create(&header_and_payload);

        return header_and_payload + Self::LINE_SEPARATOR + signature.as_str();
    }

    pub fn deserialize(classic_form: &'outer str) -> Result<JsonAccessWebToken<'vague>, ()> {
        let classic_form_parts: Vec<&'_ str> = classic_form.split(Self::LINE_SEPARATOR).collect::<Vec<&'_ str>>();

        if classic_form_parts.len() == 3 {
            if SignatureCreator::is_valid(
                (String::new() + classic_form_parts[0] + Self::LINE_SEPARATOR + classic_form_parts[1]).as_str(), classic_form_parts[2]
            ) 
            {
                return Ok(
                    JsonAccessWebToken::new_from_payload_common(
                        serde_json::from_slice::<'_, PayloadCommon>(&base64::decode(classic_form_parts[1].as_bytes()).unwrap()).unwrap()  // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
                    )?
                );
            }
        }

        return Err(());
    }
} 