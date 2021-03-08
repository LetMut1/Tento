// use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;

// use lettre_email::Email;
// use lettre_email::EmailBuilder;
// use lettre::ClientSecurity;
// use lettre::smtp::authentication::Credentials;   // TODO  uncomment
// use lettre::smtp::authentication::Mechanism;
// use lettre::smtp::ConnectionReuseParameters;
// use lettre::smtp::extension::ClientId;
// use lettre::SmtpClient;
// use lettre::SmtpTransport;
// use lettre::Transport;
// use std::convert::Into;

// pub struct EmailSender;

// impl<'outer> EmailSender {
//     pub fn send(subject: String, body: String, to: String) -> Result<(), EmailErrorKind> {
//         let email: Email;
//         match EmailBuilder::new()
//                 .subject(subject)
//                 .text(body)
//                 .from("from_changethis@yandex.ru".to_string())
//                 .to(to)
//                 .build() {
//             Ok(value) => { email = value; },
//             Err(value) => { return EmailErrorKind::Creating()}   // TODO через шке конструктор
//         };

//         // let mut smtp_transport: SmtpTransport = // TODO использовать  этот код для продакшена, изучив неоходимое. Для Prod нужен этот код, для Dev -тот, что ниже
//         //     SmtpClient::new_simple("mailhog").unwrap()
//         //         .hello_name(ClientId::Domain("TODO".to_string()))
//         //         .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO from env
//         //         .smtp_utf8(true)
//         //         .authentication_mechanism(Mechanism::Plain)// TODO
//         //         .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO 
//         //         .transport();
//         // match smtp_transport.send(email.into()) {
//         //     Ok(_value) => { return (); },
//         //     Err(ref value) => { panic!(println!("{:?}", value)); }  // TODO обработать ошибку
//         // };



//         // TODO delete
//         // TODO сделать возможность пробрасывать среду dev/prod с конфига
//         let mut smtp_transport: SmtpTransport = SmtpClient::new(("mailhog", 1025), ClientSecurity::None).unwrap().transport();
//         smtp_transport.send(email.into()).unwrap();
//         return ();
//     }
// }