use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
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

impl<'outer> EmailSender {
    pub fn send(subject: &'outer str, body: String, to: &'outer str) -> Result<(), EmailErrorKind> {
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

                //         match smtp_transport.send(email.into()) {
                //             Ok(_) => { 
                //                 return Ok(()); 
                //             },
                //             Err(error) => { 
                //                 return Err(EmailErrorKind::new_sending(error, None)); 
                //             }
                //         };
                //     },
                //     Err(error) => { 
                //         return Err(EmailErrorKind::new_sending(error, None)); 
                //     }
                // };
                // TODO uncomment }





                // TODO delete below-------------------------------------------------------------
                // TODO сделать возможность пробрасывать среду dev/prod с конфига
                let mut smtp_transport: SmtpTransport;
                match SmtpClient::new(("mailhog", 1025), ClientSecurity::None) {
                    Ok(smtp_client) => {
                        smtp_transport = smtp_client.transport();
                    },
                    Err(value) => { return Err(EmailErrorKind::new_sending(value, None)); }
                };

                match smtp_transport.send(email.into()) {
                    Ok(_) => { return Ok(()); },
                    Err(value) => { return Err(EmailErrorKind::new_sending(value, None)); }
                };
                // TODO delete under ---------------------------------------------------------------







            },
            Err(error) => { 
                return Err(EmailErrorKind::new_creating(error, None)); 
            }
        };
    }
}