use super::EmailSender;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;

use crate::infrastructure_layer::functionality::service::sender::email::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;

impl EmailSender<ApplicationUserResetPasswordToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_reset_password_token_value: &'a str,
        application_user_email: &'a str,
        application_user_device_id: &'a str,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code: {} for device {}.",
            application_user_reset_password_token_value, application_user_device_id,
        );

        Sender::<Email>::send(
            environment_configuration,
            "Reset password confirmation",
            message_body,
            application_user_email,
        )?;

        return Ok(());
    }
}
