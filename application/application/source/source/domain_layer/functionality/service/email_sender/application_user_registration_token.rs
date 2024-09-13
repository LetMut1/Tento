use std::future::Future;
use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken,
    infrastructure_layer::{
        data::{
            capture::Capture, control_type::Email, environment_configuration::environment_configuration::EnvironmentConfiguration
        },
        functionality::service::sender::Sender,
    },
};
use aggregate_error::AggregateError;
use void::Void;
impl EmailSender<ApplicationUserRegistrationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_registration_token__value: &'a str,
        application_user__email: &'a str,
        application_user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let message_body = format!(
                "Your code {} for device {}.",
                application_user_registration_token__value, application_user_device__id,
            );
            Sender::<Email>::send(
                environment_configuration,
                "Registration confirmation",
                message_body,
                application_user__email,
            ).await?;
            return Ok(());
        };
    }
}
