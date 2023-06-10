use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::sender::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;
use super::email_sender::EmailSender;

impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token_value.get(),
            application_user_device_id.get()
        );

        if let Err(mut error) = Sender::<Email>::send(
            environment_configuration,
            "Authorization confirmation",
            message_body,
            application_user_email.get()
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}