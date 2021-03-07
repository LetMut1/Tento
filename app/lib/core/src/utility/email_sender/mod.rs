use lettre_email::Email;
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::authentication::Mechanism;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::extension::ClientId;
use lettre::SmtpClient;

pub struct EmailSender;

impl<'outer> EmailSender {
    pub fn send(email: &'outer String) -> () {
        let email: Email = 
            EmailBuilder::new()
                .subject("Registration comfirmation")
                .text("sdvsdv".to_string())
                .from("from_changethis@yandex.ru")
                .to("to_changethis@yandex.ru")
                .build().unwrap();   // TODO нужно ли ошибки обрабатыват
        let smtp_client: SmtpClient = 
            SmtpClient::new_simple("TODO").unwrap()
                .hello_name(ClientId::Domain("TODO".to_string()))
                .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string()))
                .smtp_utf8(true)
                .authentication_mechanism(Mechanism::Plain)     // TODO
                .connection_reuse(ConnectionReuseParameters::NoReuse)    // TODO 
                .transport();
    } // TODO ПРАВИЛЬНый ли тип переменной?
}