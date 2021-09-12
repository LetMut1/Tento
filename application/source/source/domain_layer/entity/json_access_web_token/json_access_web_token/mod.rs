use crate::domain_layer::entity::json_access_web_token::_component::header::Header;
use crate::domain_layer::entity::json_access_web_token::_component::payload::Payload;
use std::clone::Clone;

#[derive(Clone)]
pub struct JsonAccessWebToken<'outer_a> {
    header: Header,
    payload: Payload<'outer_a>,
}

impl<'outer_a> JsonAccessWebToken<'outer_a> {
    pub fn new(
        payload: Payload<'outer_a>
    ) -> Self {
        return Self {
            header: Header::new(),
            payload
        };
    }

    pub fn get_type<'this>(
        &'this self
    ) -> &'this str {
        return self.header.get_type();
    }

    pub fn get_id<'this>(
        &'this self
    ) -> &'this str {
        return self.payload.get_id();
    }

    pub fn get_application_user_id<'this>(
        &'this self
    ) -> &'this i64 {
        return self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id<'this>(
        &'this self
    ) -> &'this str {
        return self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_expiration_time<'this>(
        &'this self
    ) -> &'this str {
        return self.payload.get_expiration_time();
    }
}