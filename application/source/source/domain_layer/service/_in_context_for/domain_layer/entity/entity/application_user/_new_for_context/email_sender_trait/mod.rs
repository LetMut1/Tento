use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use std::error::Error;

pub trait EmailSenderTrait {
    type Error: Error;

    fn send_application_user_log_in_token<'outer_a>(
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error>;

    fn send_application_user_registration_confirmation_token<'outer_a>(
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;

    fn send_application_user_reset_password_token<'outer_a>(
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error>;
}