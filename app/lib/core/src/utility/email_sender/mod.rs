use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
use lettre_email::Email;
use lettre_email::EmailBuilder;
use lettre::ClientSecurity;
use lettre::smtp::authentication::Credentials;   // TODO  uncomment
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
        let email: Email;

        match EmailBuilder::new()
                .subject(subject)
                .text(body)
                .from("from_changethis@yandex.ru".to_string())
                .to(to)
                .build() {
            Ok(value) => { email = value; },
            Err(value) => { return Err(EmailErrorKind::new_creating(value, None)); }   // TODO через шке конструктор
        };

        let mut smtp_transport: SmtpTransport;

        // TODO uncomment {  
        // match SmtpClient::new_simple("TODO") {  // TODO Для Prod нужен этот код, для Dev -тот, что ниже
        //     Ok(value) => {
        //         smtp_transport = value
        //             .hello_name(ClientId::Domain("TODO".to_string()))
        //             .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO from env
        //             .smtp_utf8(true)
        //             .authentication_mechanism(Mechanism::Plain)// TODO
        //             .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO 
        //             .transport();
        //     },
        //     Err(value) => { return Err(EmailErrorKind::new_sending(value, None)); }
        // };

        // match smtp_transport.send(email.into()) {
        //     Ok(_) => { return Ok(()); },
        //     Err(value) => { return Err(EmailErrorKind::new_sending(value, None)); }
        // };
        // TODO uncomment }




        
        // TODO delete below
        // TODO сделать возможность пробрасывать среду dev/prod с конфига
        match SmtpClient::new(("mailhog", 1025), ClientSecurity::None) {
            Ok(value) => {
                smtp_transport = value.transport();
            },
            Err(value) => { return Err(EmailErrorKind::new_sending(value, None)); }
        };

        match smtp_transport.send(email.into()) {
            Ok(_) => { return Ok(()); },
            Err(value) => { return Err(EmailErrorKind::new_sending(value, None)); }
        };
    }
}