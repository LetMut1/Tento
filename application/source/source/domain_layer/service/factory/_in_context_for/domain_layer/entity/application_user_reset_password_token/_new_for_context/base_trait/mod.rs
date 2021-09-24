use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::error::logic_error::LogicError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use std::borrow::Cow;
use std::convert::From;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error + From<LogicError>;
    type ValueGenerator: ValueGeneratorTrait;

    fn create_from_application_user<'outer_a>(
        application_user: &'outer_a ApplicationUser
    ) -> Result<ApplicationUserResetPasswordToken<'outer_a>, Self::Error> {
        return Ok(
            ApplicationUserResetPasswordToken::new(
                application_user.get_id()?,
                Cow::Borrowed(application_user.get_email()),
                <Self::ValueGenerator as ValueGeneratorTrait>::generate(),
                0
            )
        );
    }

    fn create_from_common<'outer_a>(
        common: Common<'_>,
        application_user_id: &'outer_a i64
    ) -> ApplicationUserResetPasswordToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) : (
            Cow<'_, str>,
            Cow<'_, str>,
            Cow<'_, u8>
        ) = common.into_inner();

        return ApplicationUserResetPasswordToken::new(
            application_user_id,
            Cow::Owned(application_user_email.into_owned()),
            value.into_owned(),
            wrong_enter_tries_quantity.into_owned()
        );
    }
}