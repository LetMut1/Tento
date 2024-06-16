use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            control_type::Email,
            environment_configuration::EnvironmentConfiguration,
            error::Error,
        },
        functionality::service::sender::Sender,
    },
};
impl EmailSender<ApplicationUserResetPasswordToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_reset_password_token_value: &'a str,
        application_user__email: &'a str,
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
            application_user__email,
        )?;
        return Ok(());
    }
}
