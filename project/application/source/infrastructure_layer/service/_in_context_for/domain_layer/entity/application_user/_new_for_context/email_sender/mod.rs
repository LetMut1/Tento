use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::email_sender::EmailSender as BaseEmailSender;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;

pub struct EmailSender;

impl EmailSender {     // TODO все &'static str в константы? Тогда пройтись по всему приложению и проверить, везде ли так.
    pub fn send_application_user_log_in_token<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        application_user_log_in_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_variable_resolver,
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
            return Err(error);
        }

        return Ok(());
    }

    pub fn send_application_user_registration_confirmation_token<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        application_user_registration_confirmation_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_variable_resolver,
            "Registration confirmation", 
            "Your code: ".to_string() + application_user_registration_confirmation_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
            return Err(error);
        }

        return Ok(());
    }

    pub fn send_application_user_reset_password_token<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        application_user_reset_password_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_variable_resolver,
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