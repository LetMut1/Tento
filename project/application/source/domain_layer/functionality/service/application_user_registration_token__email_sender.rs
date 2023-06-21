use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::infrastructure_layer::data::environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::sender::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;
use super::email_sender::EmailSender;

impl EmailSender<ApplicationUserRegistrationToken<'_>> {
    pub fn send<'a>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        application_user_registration_token_value: &'a ApplicationUserRegistrationToken_Value,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_registration_token_value.get(),
            application_user_device_id.get()
        );

        if let Err(mut error) = Sender::<Email>::send(
            pushable_environment_configuration,
            "Registration confirmation",
            message_body,
            application_user_email.get()
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}