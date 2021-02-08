use crate::dto::entity::entity::json_web_token::json_access_web_token::core::header::common::Common as HeaderCommon;
use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common as PayloadCommon;
use crate::entity::entity::json_web_token::json_access_web_token::JsonAccessWebToken;
use crate::util::entity::entity::json_web_token::json_access_web_token::hs512_encryptor::HS512Encoder;

pub struct SerializationFormResolver {
    hs512_encoder: HS512Encoder
}

impl<'a, 'b: 'a> SerializationFormResolver {
    pub fn new() -> Self {
        return Self {
            hs512_encoder: HS512Encoder::new()
        };
    }

    pub fn serialize(&'a self, json_access_web_token: &'a JsonAccessWebToken) -> String {
        let header_common: HeaderCommon = HeaderCommon::new_from_entity(json_access_web_token);
        let payload_common: PayloadCommon = PayloadCommon::new_from_entity(json_access_web_token);

        return self.create_classic_form(&serde_json::to_string(&header_common).unwrap(), &serde_json::to_string(&payload_common).unwrap());
    }

    pub fn deserialize(&'a self, jawt_classic_form: &'b String) -> JsonAccessWebToken {
        let jawt_parts: Vec<String> = jawt_classic_form.split(".").map(|value: &str| -> String { return value.to_owned(); }).collect();

        if self.is_valid(&jawt_parts) {
            let paylod_json_encoded: &[u8] = &base64::decode(&jawt_parts[1].as_bytes()).unwrap(); // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
            let payload_common: PayloadCommon = serde_json::from_slice(paylod_json_encoded).unwrap(); // TODO По сути, обработать ошвозможную ошибку нужно, но ее не будет по факту
            
            return JsonAccessWebToken::new_from_payload_dto_common(payload_common);
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