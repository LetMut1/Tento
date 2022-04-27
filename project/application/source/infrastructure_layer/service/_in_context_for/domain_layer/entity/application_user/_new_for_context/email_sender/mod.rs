use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::email_sender::EmailSender as BaseEmailSender;

pub struct EmailSender;

impl EmailSenderTrait for EmailSender {     // TODO все &'static str в константы? Тогда пройтись по всему приложению и проверить, везде ли так.
    type Error = ErrorAuditor;

    fn send_application_user_log_in_token<'a>(
        application_user_log_in_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), Self::Error> {
        if let Err(mut error) = BaseEmailSender::send(
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
            return Err(error);
        }

        return Ok(());
    }

    fn send_application_user_registration_confirmation_token<'a>(
        application_user_registration_confirmation_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), Self::Error> {
        if let Err(mut error) = BaseEmailSender::send(
            "Registration confirmation", 
            "Your code: ".to_string() + application_user_registration_confirmation_token_value,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
            return Err(error);
        }

        return Ok(());
    }

    fn send_application_user_reset_password_token<'a>(
        application_user_reset_password_token_value: &'a str,
        application_user_email: &'a str
    ) -> Result<(), Self::Error> {
        if let Err(mut error) = BaseEmailSender::send(
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