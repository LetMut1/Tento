use super::EmailSender;
use crate::{
    domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken,
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            capture::Capture,
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::sender::{
            Email,
            Sender,
        },
    },
};
use dedicated_crate::void::Void;
use std::future::Future;
impl EmailSender<UserResetPasswordToken<'_>> {
    pub fn repeatable_send<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        user_reset_password_token__value: &'a str,
        user__email: &'a str,
        user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let message_body = format!(
                "Your code: {} for device {}.",
                user_reset_password_token__value,
                user_device__id,
            );
            Sender::<Email>::repeatable_send(
                environment_configuration,
                "Reset password confirmation",
                message_body,
                user__email,
            )
            .await?;
            return Result::Ok(());
        };
    }
}
