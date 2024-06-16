use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            environment_configuration::EnvironmentConfiguration,
            error::Error,
        },
        functionality::service::sender::{
            email::Email,
            Sender,
        },
    },
};
impl EmailSender<ApplicationUserRegistrationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_registration_token_value: &'a str,
        application_user_email: &'a str,
        application_user_device_id: &'a str,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_registration_token_value, application_user_device_id,
        );
        Sender::<Email>::send(
            environment_configuration,
            "Registration confirmation",
            message_body,
            application_user_email,
        )?;
        return Ok(());
    }
}
