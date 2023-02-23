use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::email_sender::EmailSender as BaseEmailSender;

pub struct ApplicationUser_EmailSender;

impl ApplicationUser_EmailSender {     // TODO все &'static str в константы? Тогда пройтись по всему приложению и проверить, везде ли так.
    pub fn send_application_user_authorization_token<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_authorization_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_configuration,
            "Log in confirmation", "Your code: ".to_string() + application_user_authorization_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub fn send_application_user_registration_token<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_registration_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_configuration,
            "Registration confirmation",
            "Your code: ".to_string() + application_user_registration_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub fn send_application_user_reset_password_token<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_reset_password_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_configuration,
            "Reset password confirmation",
            "Your code: ".to_string() + application_user_reset_password_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}