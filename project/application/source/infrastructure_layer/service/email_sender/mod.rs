use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use lettre_email::EmailBuilder;
use lettre::ClientSecurity;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::authentication::Mechanism;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::extension::ClientId;
use lettre::smtp::SmtpClient;
use lettre::Transport;
use std::convert::Into;

pub struct EmailSender;

impl EmailSender {   // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(
        subject: &'a str,
        body: String,
        to: &'a str
    ) -> Result<(), ErrorAggregator> {
        let email = EmailBuilder::new().subject(subject).text(body).from("from_changethis@yandex.ru".to_string()).to(to).build()?; //TODO

        let smtp_client: SmtpClient;

        if EnvironmentVariableResolver::is_production()? {
            smtp_client = SmtpClient::new_simple("TODO")?   // TODO
            .hello_name(ClientId::Domain("TODO".to_string())) // TODO
            .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)// TODO
            .connection_reuse(ConnectionReuseParameters::NoReuse);// TODO 
        } else {
            smtp_client = SmtpClient::new(
                EnvironmentVariableResolver::get_resource_email_server_socket_address()?.as_str(), ClientSecurity::None
            )?
        }

        smtp_client.transport().send(email.into())?;

        return Ok(()); 
    }
}