use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::error::logic_error::LogicError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use std::convert::From;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error + From<LogicError>;
    type ValueGenerator: ValueGeneratorTrait;

    fn create_from_application_user<'a>(
        application_user: &'a ApplicationUser,
        device_id: &'a str
    ) -> Result<ApplicationUserLogInToken<'a>, Self::Error> {
        return Ok(
            ApplicationUserLogInToken::new(
                application_user.get_id()?,
                device_id,
                <Self::ValueGenerator as ValueGeneratorTrait>::generate(),
                0
            )
        );
    }
}