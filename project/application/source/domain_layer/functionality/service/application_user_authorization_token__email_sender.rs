use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::sender::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;
use super::email_sender::EmailSender;

impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_authorization_token_value: &'a str,
        application_user_email: &'a str,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token_value,
            application_user_device_id
        );


        if let Err(mut error) = Sender::<Email>::send(
            environment_configuration,
            "Authorization confirmation",
            message_body,
            application_user_email
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}