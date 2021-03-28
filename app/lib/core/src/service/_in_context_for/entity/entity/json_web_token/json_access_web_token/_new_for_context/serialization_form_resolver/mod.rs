use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::header::header::_new_for_context::common::Common as HeaderCommon;
use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common as PayloadCommon;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::_new_for_context::signature_encoder::SignatureEncoder;
use serde_json;

pub struct SerializationFormResolver;

impl<'outer, 'vague> SerializationFormResolver {
    const LINE_SEPARATOR: &'static str = ".";

    pub fn serialize(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> String {
        let header_common: HeaderCommon<'_> = HeaderCommon::new_from_entity(json_access_web_token);

        let payload_common: PayloadCommon<'_> = PayloadCommon::new(json_access_web_token);

        return Self::create_classic_form(
            serde_json::to_string::<HeaderCommon<'_>>(&header_common).unwrap().as_str(), 
            serde_json::to_string::<PayloadCommon<'_>>(&payload_common).unwrap().as_str()
        );
    }

    pub fn deserialize(classic_form: &'outer str) -> Result<JsonAccessWebToken<'vague>, ()> {
        let classic_form_parts: Vec<&'_ str> = classic_form.split(Self::LINE_SEPARATOR).collect::<Vec<&'_ str>>();

        if Self::is_valid(&classic_form_parts) {
            let paylod_json_encoded: &'_ [u8] = &base64::decode(classic_form_parts[1].as_bytes()).unwrap(); // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
            
            return Ok(JsonAccessWebToken::new_from_payload_common(serde_json::from_slice::<'_, PayloadCommon<'_>>(paylod_json_encoded).unwrap())?);  // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
        } else {
            return Err(());
        }
    }

    fn create_classic_form(header: &'vague str, payload: &'vague str) -> String {
        let header_and_payload: String = base64::encode(header.as_bytes()) + Self::LINE_SEPARATOR + base64::encode(payload.as_bytes()).as_str();
        
        let signature: String = SignatureEncoder::encode(&header_and_payload);

        return header_and_payload + Self::LINE_SEPARATOR + &signature;
    }

    fn is_valid(json_access_web_token_parts: &'vague Vec<&'vague str>) -> bool {
        if json_access_web_token_parts.len() == 3 {
            if SignatureEncoder::is_valid(
                (String::new() + json_access_web_token_parts[0] + Self::LINE_SEPARATOR + json_access_web_token_parts[1]).as_str(), json_access_web_token_parts[2]
            ) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
} 