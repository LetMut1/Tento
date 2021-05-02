use crate::error::main_error_kind::core::resource_error_kind::core::email_server::email_server_error_kind::EmailServerErrorKind;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use lettre_email::EmailBuilder;
use lettre::ClientSecurity;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::authentication::Mechanism;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::extension::ClientId;
use lettre::SmtpClient;
use lettre::SmtpTransport;
use lettre::Transport;
use std::convert::Into;

pub struct EmailSender;

impl<'outer_a> EmailSender {
    pub fn send(subject: &'outer_a str, body: String, to: &'outer_a str) -> Result<(), ResourceErrorKind> {
        match EmailBuilder::new()
        .subject(subject)
        .text(body)
        .from("from_changethis@yandex.ru".to_string())
        .to(to)
        .build() 
        {
            Ok(email) => { 
                // TODO uncomment {  
                // match SmtpClient::new_simple("TODO") {  // TODO Для Prod нужен этот код, для Dev -тот, что ниже
                //     Ok(smtp_client) => {
                //         let mut smtp_transport: SmtpTransport = 
                //             smtp_client
                //             .hello_name(ClientId::Domain("TODO".to_string()))
                //             .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO from env
                //             .smtp_utf8(true)
                //             .authentication_mechanism(Mechanism::Plain)// TODO
                //             .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO 
                //             .transport();

                //         if let Err(error) = smtp_transport.send(email.into()) { 
                //             return Err(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(value)));
                //         }

                //         return Ok(());   
                //     },
                //     Err(error) => { 
                //         return Err(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(value)));
                //     }
                // }
                // TODO uncomment }





                // TODO delete below-------------------------------------------------------------
                // TODO сделать возможность пробрасывать среду dev/prod с конфига
                let mut smtp_transport: SmtpTransport;
                match SmtpClient::new(("mailhog", 1025), ClientSecurity::None) {
                    Ok(smtp_client) => {
                        smtp_transport = smtp_client.transport();
                    },
                    Err(value) => { return Err(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(value))); }
                }

                match smtp_transport.send(email.into()) {
                    Ok(_) => { return Ok(()); },
                    Err(value) => { return Err(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(value))); }
                }
                // TODO delete under ---------------------------------------------------------------







            },
            Err(error) => { 
                return Err(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::EmailError(error)));
            }
        }
    }
}