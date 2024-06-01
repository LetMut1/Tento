use super::EmailSender;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::sender::email::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;
use crate::infrastructure_layer::data::error::Error;

impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value,
        application_user_email: &'a str,
        application_user_device_id: &'a str,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token_value.0.as_str(),
            application_user_device_id,
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
