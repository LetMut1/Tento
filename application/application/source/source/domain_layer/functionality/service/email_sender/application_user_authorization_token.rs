use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken,
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            control_type::Email,
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::sender::Sender,
    },
};
impl EmailSender<ApplicationUserAuthorizationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_authorization_token__value: &'a str,
        application_user__email: &'a str,
        application_user_device__id: &'a str,
    ) -> Result<(), AggregateError> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_authorization_token__value, application_user_device__id,
        );
        Sender::<Email>::send(
            environment_configuration,
            "Authorization confirmation",
            message_body,
            application_user__email,
        )?;
        return Ok(());
    }
}
