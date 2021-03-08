use lettre_email::Email;
use lettre_email::EmailBuilder;
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
    pub fn send(subject: String, body: String, to: String) -> () {
        let email: Email = 
            EmailBuilder::new()
                .subject(subject)
                .text(body)
                .from("from_changethis@yandex.ru".to_string())
                .to(to)
                .build().unwrap();   // TODO нужно ли ошибки обрабатыват. Что, если плохой to?
        let mut smtp_transport: SmtpTransport = 
            SmtpClient::new_simple("TODO").unwrap()
                .hello_name(ClientId::Domain("TODO".to_string()))
                .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO from env
                .smtp_utf8(true)
                .authentication_mechanism(Mechanism::Plain)// TODO
                .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO 
                .transport();
        match smtp_transport.send(email.into()) {
            Ok(_value) => { return (); },
            Err(ref value) => { panic!(println!("{:?}", value)); }  // TODO обработать ошибку
        };
    }
}