use std::future::Future;
use super::EmailSender;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken,
    infrastructure_layer::{
        data::{
            capture::Capture, environment_configuration::environment_configuration::EnvironmentConfiguration
        },
        functionality::service::sender::Sender,
    },
};
use crate::infrastructure_layer::functionality::service::sender::email::Email;
use aggregate_error::AggregateError;
use void::Void;
impl EmailSender<ApplicationUserResetPasswordToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_reset_password_token__value: &'a str,
        application_user__email: &'a str,
        application_user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let message_body = format!(
                "Your code: {} for device {}.",
                application_user_reset_password_token__value, application_user_device__id,
            );
            Sender::<Email>::send(
                environment_configuration,
                "Reset password confirmation",
                message_body,
                application_user__email,
            ).await?;
            return Ok(());
        };
    }
}
