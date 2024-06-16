use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken,
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
impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_authorization_token_value: &'a str,
        application_user_email: &'a str,
        application_user_device_id: &'a str,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token_value, application_user_device_id,
        );
        Sender::<Email>::send(
            environment_configuration,
            "Authorization confirmation",
            message_body,
            application_user_email,
        )?;
        return Ok(());
    }
}
