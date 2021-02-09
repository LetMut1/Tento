use crate::dto::entity::entity::json_web_token::json_access_web_token::core::header::common::Common as HeaderCommon;
use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common as PayloadCommon;
use crate::entity::entity::json_web_token::json_access_web_token::JsonAccessWebToken;
use crate::util::entity::entity::json_web_token::json_access_web_token::hs512_encoder::HS512Encoder;
use std::option::Option;

pub struct SerializationFormResolver<'a> {
    hs512_encoder: HS512Encoder,
    payload_common: Option<PayloadCommon<'a>>
}

impl<'a, 'b: 'a> SerializationFormResolver<'a> {
    pub fn new() -> Self {
        return Self {
            hs512_encoder: HS512Encoder::new(),
            payload_common: None
        };
    }

    pub fn serialize(&'a self, json_access_web_token: &'b JsonAccessWebToken<'a, 'b>) -> String {
        let header_common: HeaderCommon<'a> = HeaderCommon::new_from_entity(json_access_web_token);
        let payload_common: PayloadCommon<'a> = PayloadCommon::new_from_entity(json_access_web_token);

        return self.create_classic_form(
            &serde_json::to_string::<HeaderCommon<'a>>(&header_common).unwrap(), 
            &serde_json::to_string::<PayloadCommon<'a>>(&payload_common).unwrap()
        );
    }

    pub fn deserialize(&'a mut self, jawt_classic_form: &'b String) -> &'a PayloadCommon<'a> {
        let jawt_parts: Vec<String> = jawt_classic_form.split(".").map(|value: &str| -> String { return value.to_owned(); }).collect();
        if self.is_valid(&jawt_parts) {
            let paylod_json_encoded: &[u8] = &base64::decode(jawt_parts[1].as_bytes()).unwrap(); // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
            self.payload_common = Some(serde_json::from_slice::<'_, PayloadCommon<'a>>(paylod_json_encoded).unwrap());  // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту

            return (&self.payload_common).as_ref().unwrap();
        } else {
            panic!("Выбрасываем исключение, то есть, возвращаем Резалт с кастомной ошибкой");   // TODO 
        }
    }

    fn create_classic_form(&'a self, header: &'a String, payload: &'a String) -> String {
        let header_and_payload: String = base64::encode(header.as_bytes()) + "." + &base64::encode(payload.as_bytes());
        let signature: String = self.hs512_encoder.encode(&header_and_payload);

        return header_and_payload + "." + &signature;
    }

    fn is_valid(&'a self, jawt_parts: &'a Vec<String>) -> bool {
        if jawt_parts.len() == 3 {
            if self.hs512_encoder.hash_is_valid(&("".to_string() + &jawt_parts[0] + "." + &jawt_parts[1]), &jawt_parts[2]) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
} 