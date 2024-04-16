use super::EmailSender;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::sender::email::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;
use crate::infrastructure_layer::data::error::Error;

impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token_value.0.as_str(),
            application_user_device_id.0.as_str()
        );

        if let Err(mut error) = Sender::<Email>::send(
            "Authorization confirmation",
            message_body,
            application_user_email.0.as_str(),
        ) {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error);
        }

        return Ok(());
    }
}
