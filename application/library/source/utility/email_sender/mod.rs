use crate::error::base_error::_core::run_time_error::_core::resource_error::_core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use crate::error::base_error::_core::run_time_error::_core::resource_error::resource_error::ResourceError;
use crate::error::base_error::_core::run_time_error::run_time_error::RunTimeError;
use crate::error::base_error::base_error::BaseError;
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
    pub fn send<'outer_a>(subject: &'outer_a str, body: String, to: &'outer_a str) -> Result<(), BaseError> {
        let email: Email = match EmailBuilder::new().subject(subject).text(body).from("from_changethis@yandex.ru".to_string()).to(to).build() //TODO
        {
            Ok(email) => email,
            Err(email_error) => { 
                return Err(BaseError::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::EmailError(email_error)))));
            }
        };

        if EnvironmentVariableResolver::is_production()? {
            let smtp_client: SmtpClient = match SmtpClient::new_simple("TODO") { // TODO
                Ok(smtp_client) => smtp_client,
                Err(smtp_error) => { 
                    return Err(BaseError::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::SmtpError(smtp_error)))));
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
                return Err(BaseError::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::SmtpError(smtp_error)))));
            }  
        } else {
            let mut smtp_transport: SmtpTransport = match SmtpClient::new(
                EnvironmentVariableResolver::get_resource_email_server_socket_address()?.as_str(), ClientSecurity::None
            ) 
            {
                Ok(smtp_client) => smtp_client.transport(),
                Err(smtp_error) => { 
                    return Err(BaseError::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::SmtpError(smtp_error))))); 
                }
            };
    
            if let Err(smtp_error) = smtp_transport.send(email.into()) {
                return Err(BaseError::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::SmtpError(smtp_error))))); 
            }
        }

        return Ok(()); 
    }
}