use super::email_sender::EmailSender;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::sender::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;

impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token_value.get(),
            application_user_device_id.get()
        );

        if let Err(mut error) = Sender::<Email>::send(
            pushable_environment_configuration,
            "Authorization confirmation",
            message_body,
            application_user_email.get(),
        ) {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }

        return Ok(());
    }
}
