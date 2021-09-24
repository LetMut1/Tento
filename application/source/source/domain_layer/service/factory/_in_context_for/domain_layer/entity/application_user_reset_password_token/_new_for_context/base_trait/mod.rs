use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::error::logic_error::LogicError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
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
}