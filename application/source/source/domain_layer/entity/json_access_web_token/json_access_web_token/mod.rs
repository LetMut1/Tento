use crate::domain_layer::entity::json_access_web_token::_component::header::header::Header;
use crate::domain_layer::entity::json_access_web_token::_component::payload::payload::Payload;
use std::clone::Clone;

#[derive(Clone)]
pub struct JsonAccessWebToken<'outer_a> {       // TODO добавть Сигнатуру поле. Создавать его при создании токена в Фактори, отсюда брать три куска и клеить их в сервисе в base64
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

    pub fn is_expired<'this>(
        &'this self
    ) -> bool {
        return true;
        return false;

        // return !DateTimeManipulator::is_greater_or_equal_than_now(&self.payload.get_exp().get_value());
    }

    pub fn get_alg<'this>(
        &'this self
    ) -> &'this str {
        return self.header.get_alg();
    }

    pub fn get_typ<'this>(
        &'this self
    ) -> &'this str {
        return self.header.get_typ();
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

    pub fn get_exp<'this>(
        &'this self
    ) -> &'this str {
        return self.payload.get_exp();
    }
}