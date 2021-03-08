use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::header::header::_new_for_context::common::Common as HeaderCommon;
use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common as PayloadCommon;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::_new_for_context::signature_creator::SignatureCreator;
use serde_json;

pub struct SerializationFormResolver;

impl<'this, 'outer: 'this> SerializationFormResolver {
    const LINE_SEPARATOR: &'static str = ".";

    pub fn serialize(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> String {
        let header_common: HeaderCommon<'_> = HeaderCommon::new_from_entity(json_access_web_token);

        let payload_common: PayloadCommon<'_> = PayloadCommon::new_from_entity(json_access_web_token);

        return Self::create_classic_form(
            &serde_json::to_string::<HeaderCommon<'_>>(&header_common).unwrap(), 
            &serde_json::to_string::<PayloadCommon<'_>>(&payload_common).unwrap()
        );
    }

    pub fn deserialize(classic_form: &'outer String) -> PayloadCommon<'this> {
        let parts: Vec<String> = classic_form.split(Self::LINE_SEPARATOR).map(|value: &'_ str| -> String { return value.to_string(); }).collect();

        if Self::is_valid(&parts) {
            let paylod_json_encoded: &'_ [u8] = &base64::decode(parts[1].as_bytes()).unwrap(); // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
            
            return serde_json::from_slice::<'_, PayloadCommon<'this>>(paylod_json_encoded).unwrap();  // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
        } else {
            panic!("Выбрасываем исключение, то есть, возвращаем Резалт с кастомной ошибкой");   // TODO 
        }
    }

    fn create_classic_form(header: &'this String, payload: &'this String) -> String {
        let header_and_payload: String = base64::encode(header.as_bytes()) + Self::LINE_SEPARATOR + &base64::encode(payload.as_bytes());
        
        let signature: String = SignatureCreator::encode(&header_and_payload);

        return header_and_payload + Self::LINE_SEPARATOR + &signature;
    }

    fn is_valid(json_access_web_token_parts: &'this Vec<String>) -> bool {
        if json_access_web_token_parts.len() == 3 {
            if SignatureCreator::hash_is_valid(
                &(String::new() + &json_access_web_token_parts[0] + Self::LINE_SEPARATOR + &json_access_web_token_parts[1]), &json_access_web_token_parts[2]
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