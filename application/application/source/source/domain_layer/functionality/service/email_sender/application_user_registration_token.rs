use super::EmailSender;
use crate::{
    domain_layer::data::entity::user_registration_token::UserRegistrationToken,
    infrastructure_layer::{
        data::{
            capture::Capture,
            environment_configuration::environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::sender::{
            email::Email,
            Sender,
        },
    },
};
use aggregate_error::AggregateError;
use std::future::Future;
use void::Void;
impl EmailSender<UserRegistrationToken<'_>> {
    pub fn repeatable_send<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_registration_token__value: &'a str,
        application_user__email: &'a str,
        application_user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let message_body = format!(
                "Your code {} for device {}.",
                application_user_registration_token__value, application_user_device__id,
            );
            Sender::<Email>::repeatable_send(
                environment_configuration,
                "Registration confirmation",
                message_body,
                application_user__email,
            )
            .await?;
            return Result::Ok(());
        };
    }
}
