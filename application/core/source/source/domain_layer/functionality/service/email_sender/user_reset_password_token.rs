use {
    crate::{
        domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken,
        infrastructure_layer::{
            data::{
                aggregate_error::AggregateError,
                environment_configuration::run_server::EmailServer,
            },
            functionality::service::sender::{
                Email,
                Sender,
            },
        },
    },
    super::EmailSender,
    std::future::Future,
};
impl EmailSender<UserResetPasswordToken<'_>> {
    pub fn repeatable_send<'a>(
        email_server: &'static EmailServer,
        user_reset_password_token__value: &'a str,
        user__email: &'a str,
        user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let message_body = format!(
                "Your code: {} for device {}.",
                user_reset_password_token__value,
                user_device__id,
            );
            Sender::<Email>::repeatable_send(
                email_server,
                "Reset password confirmation",
                message_body,
                user__email,
            )
            .await?;
            return Result::Ok(());
        };
    }
}
