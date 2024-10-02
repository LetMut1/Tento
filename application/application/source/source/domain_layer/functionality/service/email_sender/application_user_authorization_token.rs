use super::EmailSender;
use crate::{
    domain_layer::data::entity::user_authorization_token::UserAuthorizationToken,
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
impl EmailSender<UserAuthorizationToken<'_>> {
    pub fn repeatable_send<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_authorization_token__value: &'a str,
        user__email: &'a str,
        user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let message_body = format!(
                "Your code {} for device {}.",
                application_user_authorization_token__value, user_device__id,
            );
            Sender::<Email>::repeatable_send(
                environment_configuration,
                "Authorization confirmation",
                message_body,
                user__email,
            )
            .await?;
            return Result::Ok(());
        };
    }
}
