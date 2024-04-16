use super::EmailSender;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Auditor;
use crate::infrastructure_layer::data::error_auditor::Error;

use crate::infrastructure_layer::functionality::service::sender::email::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;

impl EmailSender<ApplicationUserResetPasswordToken<'_>> {
    pub fn send<'a>(
        application_user_reset_password_token_value: &'a ApplicationUserResetPasswordToken_Value,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code: {} for device {}.",
            application_user_reset_password_token_value.0.as_str(),
            application_user_device_id.0.as_str(),
        );

        if let Err(mut error) = Sender::<Email>::send(
            "Reset password confirmation",
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
