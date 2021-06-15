use crate::error::main_error_kind::core::run_time_error_kind::core::resource_error_kind::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error_kind::EmailServerErrorKind;
use crate::error::main_error_kind::core::run_time_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::error::main_error_kind::core::run_time_error_kind::run_time_error_kind::RunTimeErrorKind;
use crate::utility::environment_variable_resolver::EnvironmentVariableResolver;
use lettre_email::Email;
use lettre_email::EmailBuilder;
use lettre::ClientSecurity;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::authentication::Mechanism;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::extension::ClientId;
use lettre::smtp::SmtpClient;
use lettre::SmtpTransport;
use lettre::Transport;
use std::convert::Into;

pub struct EmailSender;

impl EmailSender {   // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'outer_a>(subject: &'outer_a str, body: String, to: &'outer_a str) -> Result<(), RunTimeErrorKind> {
        let email: Email = match EmailBuilder::new().subject(subject).text(body).from("from_changethis@yandex.ru".to_string()).to(to).build() //TODO
        {
            Ok(email) => email,
            Err(email_error) => { 
                return Err(RunTimeErrorKind::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::EmailError(email_error))));
            }
        };

        if EnvironmentVariableResolver::is_production() {
            let smtp_client: SmtpClient = match SmtpClient::new_simple("TODO") { // TODO
                Ok(smtp_client) => smtp_client,
                Err(smtp_error) => { 
                    return Err(RunTimeErrorKind::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error))));
                }
            };

            let mut smtp_transport: SmtpTransport = smtp_client
                .hello_name(ClientId::Domain("TODO".to_string())) // TODO
                .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO
                .smtp_utf8(true)
                .authentication_mechanism(Mechanism::Plain)// TODO
                .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO 
                .transport();

            if let Err(smtp_error) = smtp_transport.send(email.into()) { 
                return Err(RunTimeErrorKind::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error))));
            }  
        } else {
            let mut smtp_transport: SmtpTransport = match SmtpClient::new(
                EnvironmentVariableResolver::get_resource_email_server_socket_address().as_str(), ClientSecurity::None
            ) 
            {
                Ok(smtp_client) => smtp_client.transport(),
                Err(smtp_error) => { 
                    return Err(RunTimeErrorKind::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error)))); 
                }
            };
    
            if let Err(smtp_error) = smtp_transport.send(email.into()) {
                return Err(RunTimeErrorKind::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error)))); 
            }
        }

        return Ok(()); 
    }
}