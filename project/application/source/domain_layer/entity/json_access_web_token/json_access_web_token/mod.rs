use crate::domain_layer::entity::json_access_web_token::_component::header::Header;
use crate::domain_layer::entity::json_access_web_token::_component::payload::Payload;
use std::clone::Clone;

#[derive(Clone)]
pub struct JsonAccessWebToken<'a> {
    header: Header,
    payload: Payload<'a>,
}

impl<'a> JsonAccessWebToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 30;

    pub fn new(
        payload: Payload<'a>
    ) -> Self {
        return Self {
            header: Header::new(),
            payload
        };
    }

    pub fn get_type<'b>(
        &'b self
    ) -> &'b str {
        return self.header.get_type();
    }

    pub fn get_id<'b>(
        &'b self
    ) -> &'b str {
        return self.payload.get_json_access_web_token_id();
    }

    pub fn get_application_user_id<'b>(
        &'b self
    ) -> &'b i64 {
        return self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id<'b>(
        &'b self
    ) -> &'b str {
        return self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_expiration_time<'b>(
        &'b self
    ) -> &'b str {
        return self.payload.get_json_access_web_token_expiration_time();
    }
}