use super::Sender;
use crate::infrastructure_layer::data::{
    alternative_workflow::{
        AlternativeWorkflow,
        ResultConverter,
    },
    auditor::{
        Auditor,
        Backtrace,
    },
    control_type::Email,
    environment_configuration::EnvironmentConfiguration,
};
use lettre::{
    smtp::SmtpClient,
    ClientSecurity,
    Transport,
};
use lettre_email::EmailBuilder;
use std::{
    convert::Into,
    net::ToSocketAddrs,
};
impl Sender<Email> {
    //TODO  ASYNC client // TODO Возможно, сразу можно положить объект в константу.  // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(environment_configuration: &'a EnvironmentConfiguration, subject: &'a str, body: String, to: &'a str) -> Result<(), AlternativeWorkflow> {
        let email = EmailBuilder::new() //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build()
            .into_internal_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        // TODO В static OnceLock
        let mut email_server_socket_address_registry = environment_configuration.resource.email_server.socket_address.to_socket_addrs().into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let email_server_socket_address = match email_server_socket_address_registry.next() {
            Some(email_server_socket_address_) => email_server_socket_address_,
            None => {
                return Err(
                    AlternativeWorkflow::new_internal_logic(
                        "Invalid socket address.",
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };
        let smtp_client = SmtpClient::new(
            &email_server_socket_address,
            ClientSecurity::None,
        )
        .into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        smtp_client.transport().send(email.into()).into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(());
    }
}
