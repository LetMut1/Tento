use crate::dto::entity::entity::json_web_token::json_access_web_token::core::header::common::Common as HeaderCommon;
use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common as PayloadCommon;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::utility::entity::entity::json_web_token::json_access_web_token::hs512_encoder::HS512Encoder;

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

    pub fn deserialize(jawt_classic_form: &'outer String) -> PayloadCommon<'this> {
        let jawt_parts: Vec<String> = jawt_classic_form.split(Self::LINE_SEPARATOR).map(|value: &'_ str| -> String { return value.to_string(); }).collect();
        if Self::is_valid(&jawt_parts) {
            let paylod_json_encoded: &'_ [u8] = &base64::decode(jawt_parts[1].as_bytes()).unwrap(); // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
            
            return serde_json::from_slice::<'_, PayloadCommon<'this>>(paylod_json_encoded).unwrap();  // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
        } else {
            panic!("Выбрасываем исключение, то есть, возвращаем Резалт с кастомной ошибкой");   // TODO 
        }
    }

    fn create_classic_form(header: &'this String, payload: &'this String) -> String {
        let header_and_payload: String = base64::encode(header.as_bytes()) + Self::LINE_SEPARATOR + &base64::encode(payload.as_bytes());
        let signature: String = HS512Encoder::encode(&header_and_payload);

        return header_and_payload + Self::LINE_SEPARATOR + &signature;
    }

    fn is_valid(jawt_parts: &'this Vec<String>) -> bool {
        if jawt_parts.len() == 3 {
            if HS512Encoder::hash_is_valid(&("".to_string() + &jawt_parts[0] + Self::LINE_SEPARATOR + &jawt_parts[1]), &jawt_parts[2]) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
} 