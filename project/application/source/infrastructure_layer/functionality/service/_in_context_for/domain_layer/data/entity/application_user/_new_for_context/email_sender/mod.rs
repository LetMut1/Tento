use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::email_sender::EmailSender as BaseEmailSender;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct EmailSender;

impl EmailSender {     // TODO все &'static str в константы? Тогда пройтись по всему приложению и проверить, везде ли так.
    pub fn send_application_user_log_in_token<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_log_in_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_configuration_resolver,
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
            return Err(error);
        }

        return Ok(());
    }

    pub fn send_application_user_registration_confirmation_token<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_registration_confirmation_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_configuration_resolver,
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
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_reset_password_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = BaseEmailSender::send(
            environment_configuration_resolver,
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