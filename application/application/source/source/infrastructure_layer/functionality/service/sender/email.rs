use super::Sender;
use crate::infrastructure_layer::data::{
    control_type::Email,
    environment_configuration::environment_configuration::EnvironmentConfiguration,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use lettre::{
    smtp::SmtpClient,
    ClientSecurity,
    Transport,
};
use lettre_email::EmailBuilder;
use std::convert::Into;
impl Sender<Email> {
    //TODO  ASYNC client // TODO сразу можно положить объект в статику.  // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(environment_configuration: &'a EnvironmentConfiguration, subject: &'a str, body: String, to: &'a str) -> Result<(), AggregateError> {
        let email = EmailBuilder::new() //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build()
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        let smtp_client = SmtpClient::new(
            &environment_configuration.resource.email_server.socket_address,
            ClientSecurity::None,
        )
        .into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        smtp_client.transport().send(email.into()).into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(());
    }
}
