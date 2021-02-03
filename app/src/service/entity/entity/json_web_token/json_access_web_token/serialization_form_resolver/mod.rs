use crate::entity::entity::json_web_token::json_access_web_token::JsonAccessWebToken;
use crate::util::entity::entity::json_web_token::json_access_web_token::hs512_encryptor::HS512Encoder;
use json::JsonValue;
use json::object;

pub struct SerializationFormResolver {
    hs512_encoder: HS512Encoder
}

impl<'a> SerializationFormResolver {
    pub fn new() -> Self {
        return Self {
            hs512_encoder: HS512Encoder::new()
        };
    }

    pub fn serialize(&'a self, json_access_web_token: &'a JsonAccessWebToken) -> String {
        let header: JsonValue = object! {
            alg: json_access_web_token.get_header().get_alg().get_value(),
            typ: json_access_web_token.get_header().get_typ().get_value()
        };

        let payload: JsonValue = object! {
            user_id: json_access_web_token.get_payload().get_user_id().get_value().to_string(),
            device_id: json_access_web_token.get_payload().get_device_id().get_value().clone(),
            json_refresh_web_token_value: json_access_web_token.get_payload().get_value().get_value().to_string(),
            exp: json_access_web_token.get_payload().get_exp().get_value().to_rfc3339()
        };

        return self.create_classic_form(&header.dump(), &payload.dump());
    }

    pub fn deserialize(&'a self, jawt_classic_form: &'a String) -> JsonAccessWebToken {       // TODO Result c кастомной ошибкой, плюс обработка ошибок далее
        let jawt_parts: Vec<String> = jawt_classic_form.split(".").map(|value: &str| -> String { return value.to_string(); }).collect();

        if self.is_valid(&jawt_parts) {
            
        } else {
            // TODO выбрасывать исключение
            panic!("TODO");
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