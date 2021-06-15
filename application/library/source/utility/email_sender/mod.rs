use crate::error::main_error_kind::core::run_time_error::core::resource_error_kind::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error_kind::EmailServerErrorKind;
use crate::error::main_error_kind::core::run_time_error::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::error::main_error_kind::core::run_time_error::run_time_error::RunTimeError;
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

impl EmailSender {
    pub fn send<'outer_a>(subject: &'outer_a str, body: String, to: &'outer_a str) -> Result<(), RunTimeError> {
        let email: Email = match EmailBuilder::new()
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build() 
        {
            Ok(email) => email,
            Err(email_error) => { 
                return Err(RunTimeError::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::EmailError(email_error))));
            }
        };

        // TODO uncomment {  
        // let smtp_client: SmtpClient = match SmtpClient::new_simple("TODO") {  // TODO Для Prod нужен этот код, для Dev -тот, что ниже
        //     Ok(smtp_client) => smtp_client,
        //     Err(smtp_error) => { 
        //         return Err(RunTimeError::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error))));
        //     }
        // };

        // let mut smtp_transport: SmtpTransport = smtp_client
        //     .hello_name(ClientId::Domain("TODO".to_string()))
        //     .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO from env
        //     .smtp_utf8(true)
        //     .authentication_mechanism(Mechanism::Plain)// TODO
        //     .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO 
        //     .transport();

        // if let Err(smtp_error) = smtp_transport.send(email.into()) { 
        //     return Err(RunTimeError::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error))));
        // }

        // return Ok(());   
        // TODO uncomment }





        // TODO delete below-------------------------------------------------------------
        // TODO сделать возможность пробрасывать среду dev/prod с конфига
        let mut smtp_transport: SmtpTransport = match SmtpClient::new(("mailhog", 1025), ClientSecurity::None) {    // TODO env
            Ok(smtp_client) => smtp_client.transport(),
            Err(smtp_error) => { 
                return Err(RunTimeError::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error)))); 
            }
        };

        if let Err(smtp_error) = smtp_transport.send(email.into()) {
            return Err(RunTimeError::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(smtp_error)))); 
        }

        return Ok(()); 
        // TODO delete under ---------------------------------------------------------------
    }
}