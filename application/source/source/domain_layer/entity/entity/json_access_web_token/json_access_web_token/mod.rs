use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_access_web_token::_component::header::header::Header;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::exp::Exp;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::id::Id;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::payload::Payload;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::proxed_type::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use std::clone::Clone;
use super::_component::header::_component::alg::Alg;
use super::_component::header::_component::typ::Typ;

#[derive(Clone)]
pub struct JsonAccessWebToken<'outer_a> {
    payload: Payload<'outer_a>
}

impl<'outer_a> JsonAccessWebToken<'outer_a> {
    const HEADER: Header = Header::new();

    pub fn new(payload: Payload<'outer_a>) -> Self {
        return Self {
            payload
        };
    }

    pub fn is_expired<'this>(&'this self) -> bool {
        return !DateTimeManipulator::is_greater_or_equal_than_now(&self.payload.get_exp().get_value());
    }

    pub fn get_alg<'this>(&'this self) -> &'this Alg {
        return Self::HEADER.get_alg();
    }

    pub fn get_typ<'this>(&'this self) -> &'this Typ {
        return Self::HEADER.get_typ();
    }

    pub fn get_id<'this>(&'this self) -> &'this Id {
        return &self.payload.get_id();
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id<'this>(&'this self) -> &'this ApplicationUserLogInTokenDeviceId {
        return &self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_exp<'this>(&'this self) -> &'this Exp {
        return &self.payload.get_exp();
    }
}