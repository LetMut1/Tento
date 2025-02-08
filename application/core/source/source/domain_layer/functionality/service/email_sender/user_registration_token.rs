use {
    super::EmailSender,
    crate::{
        domain_layer::data::entity::user_registration_token::UserRegistrationToken,
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
    std::future::Future,
};
impl EmailSender<UserRegistrationToken<'_>> {
    pub fn repeatable_send<'a>(
        email_server: &'static EmailServer,
        user_registration_token__value: &'a str,
        user__email: &'a str,
        user_device__id: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let message_body = format!(
                "Your code {} for device {}.",
                user_registration_token__value,
                user_device__id,
            );
            Sender::<Email>::repeatable_send(
                email_server,
                "Registration confirmation",
                message_body,
                user__email,
            )
            .await?;
            return Result::Ok(());
        };
    }
}
