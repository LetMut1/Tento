use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use std::error::Error;
use crate::domain_layer::error::logic_error::LogicError;
use std::convert::From;
use std::borrow::Cow;
use uuid::Uuid;

pub trait BaseTrait {
    type Error: Error + From<LogicError>;

    fn new_from_application_user<'outer_a>(
        application_user: &'outer_a ApplicationUser,
        device_id: &'outer_a str
    ) -> Result<ApplicationUserLogInToken<'outer_a>, Self::Error> {
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

    fn new_from_common<'outer_a>(
        common: Common<'_>,
        application_user_id: &'outer_a i64,
        device_id: &'outer_a str
    ) -> ApplicationUserLogInToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) : (
            Cow<'_, str>,
            Cow<'_, str>,
            Cow<'_, u8>
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