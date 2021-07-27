use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use std::borrow::Cow;
use std::clone::Clone;
use super::_core::exp::Exp;
use super::_core::id::Id;

#[derive(Clone)]
pub struct Payload<'outer_a> {
    id: Cow<'outer_a, Id>,
    application_user_id: Cow<'outer_a, ApplicationUserId>,
    application_user_log_in_token_device_id: Cow<'outer_a, ApplicationUserLogInTokenDeviceId>,
    exp: Exp
}

impl<'outer_a> Payload<'outer_a> {
    pub fn new(
        id: Cow<'outer_a, Id>,
        application_user_id: Cow<'outer_a, ApplicationUserId>,
        application_user_log_in_token_device_id: Cow<'outer_a, ApplicationUserLogInTokenDeviceId>,
        exp: Exp
    ) -> Self {
        return Self {
            id, application_user_id, application_user_log_in_token_device_id, exp
        };
    }

    pub fn get_id<'this>(&'this self) -> &'this Id {
        return &self.id;
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return self.application_user_id.as_ref();
    }

    pub fn get_application_user_log_in_token_device_id<'this>(&'this self) -> &'this ApplicationUserLogInTokenDeviceId {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_exp<'this>(&'this self) -> &'this Exp {
        return &self.exp;
    }
}