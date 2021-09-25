use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use std::error::Error;

pub trait EmailSenderTrait {
    type Error: Error;

    fn send_application_user_log_in_token<'a>(
        application_user_log_in_token: &'a ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error>;

    fn send_application_user_registration_confirmation_token<'a>(
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;

    fn send_application_user_reset_password_token<'a>(
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error>;
}