use crate::data_transfer_object::_in_context_for::entity::entity::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use std::borrow::Cow;
use std::clone::Clone;
use super::core::exp::Exp;
use super::core::id::Id;

#[derive(Clone)]
pub struct Payload<'outer_a> {
    id: Cow<'outer_a, Id>,
    application_user_id: Cow<'outer_a, ApplicationUserId>,
    application_user_log_in_token_device_id: Cow<'outer_a, ApplicationUserLogInTokenDeviceId>,
    exp: Exp
}

impl<'outer_a> Payload<'outer_a> {
    pub fn new(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Self {
        return Self {
            id: Cow::Borrowed(json_refresh_web_token.get_json_access_web_token_id()),
            application_user_id: Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
            application_user_log_in_token_device_id: Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
            exp: Exp::new()
        };
    }

    pub fn new_from_common(common: Common) -> Result<Self, InvalidArgumentError> {
        return Ok (
            Self {
                id: Cow::Owned(Id::new_from_string(common.json_access_web_token_id)?),
                application_user_id: Cow::Owned(ApplicationUserId::new_from_string(common.application_user_id)?),
                application_user_log_in_token_device_id: Cow::Owned(ApplicationUserLogInTokenDeviceId::new_from_string(common.application_user_log_in_token_device_id)?),
                exp: Exp::new_from_str(common.exp.as_str())
            }
        );
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