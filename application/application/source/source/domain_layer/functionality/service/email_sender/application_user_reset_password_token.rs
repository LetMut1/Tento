use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken,
    infrastructure_layer::{
        data::{
            alternative_workflow::AlternativeWorkflow,
            control_type::Email,
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::sender::Sender,
    },
};
impl EmailSender<ApplicationUserResetPasswordToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_reset_password_token__value: &'a str,
        application_user__email: &'a str,
        application_user_device__id: &'a str,
    ) -> Result<(), AlternativeWorkflow> {
        let message_body = format!(
            "Your code: {} for device {}.",
            application_user_reset_password_token__value, application_user_device__id,
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
