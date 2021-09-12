use crate::domain_layer::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Base;

impl Base {
    pub fn new_from_application_user<'outer_a>(
        application_user: &'outer_a ApplicationUser<'_>,
        device_id: &'outer_a str
    ) -> Result<ApplicationUserLogInToken<'outer_a>, BaseError> {
        return Ok(
            ApplicationUserLogInToken::new(
                application_user.get_id()?,
                device_id,
                Cow::Borrowed(application_user.get_email()),
                Uuid::new_v4().to_string(),       // TODO создать генератор значения + метода Рефреш в модели
                0
            )
        );
    }

    pub fn new_from_common<'outer_a>(
        common: Common<'_>,
        application_user_id: &'outer_a i64,
        device_id: &'outer_a str
    ) -> ApplicationUserLogInToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) = common.into_inner();

        return ApplicationUserLogInToken::new(
            application_user_id,
            device_id,
            Cow::Owned(application_user_email.into_owned()),
            value.into_owned(),
            wrong_enter_tries_quantity.into_owned()
        );
    }
}