use super::Sender;
use crate::infrastructure_layer::data::{
    capture::Capture,
    environment_configuration::EnvironmentConfiguration,
};
use aggregate_error::{
    AggregateError,
    // Backtrace,
    // ResultConverter,
};
use std::future::Future;
use void::Void;
// use lettre::{
//     message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
//     AsyncTransport, Message, Tokio1Executor,
// };
// use std::convert::Into;
pub struct Email;
impl Sender<Email> {
    pub fn repeatable_send<'a>(
        _environment_configuration: &'static EnvironmentConfiguration,
        _subject: &'a str,
        _body: String,
        _to: &'a str,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            // TODO сделать посторяему отправку при ошибке на количество времени (отправлять через секунду, пока не выйдет время) или раз.
            return Result::Ok(());
        };
    }
}
// // TODO https://github.com/lettre/lettre/blob/master/examples/tokio1_smtp_tls.rs
// let message = Message::builder()
// .from(
//     "YourDaddy@yandex.ru".parse().into_logic(
//         Backtrace::new(
//             line!(),
//             file!(),
//         ),
//     )?,
// )
// .to(
//     to.parse().into_logic(
//         Backtrace::new(
//             line!(),
//             file!(),
//         ),
//     )?,
// )
// .subject(subject)
// .body(body)
// .into_logic(
//     Backtrace::new(
//         line!(),
//         file!(),
//     ),
// )?;
// let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());
// let mailer: AsyncSmtpTransport<Tokio1Executor> =
//     AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();
// match mailer.send(message).await {
//     Result::Ok(_) => println!("Email sent successfully!"),
//     Result::Err(e) => panic!("Could not send email: {e:?}"),
// }
