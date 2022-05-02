use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use std::error::Error;


pub trait EmailSenderTrait {
    type Error: Error;

    fn send_application_user_log_in_token<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        application_user_log_in_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), Self::Error>;

    fn send_application_user_registration_confirmation_token<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        application_user_registration_confirmation_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), Self::Error>;

    fn send_application_user_reset_password_token<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        application_user_reset_password_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), Self::Error>;
}