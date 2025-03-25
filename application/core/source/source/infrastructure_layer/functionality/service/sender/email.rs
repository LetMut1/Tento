use {
    super::Sender,
    crate::infrastructure_layer::data::aggregate_error::AggregateError,
    crate::infrastructure_layer::data::environment_configuration::run_server::EmailServer,
    std::future::Future,
    // lettre::{
    //     message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    //     AsyncTransport, Message, Tokio1Executor,
    // },
    // std::convert::Into,
};
pub struct Email;
impl Sender<Email> {
    pub fn send<'a>(_email_server: &'static EmailServer, _subject: &'a str, _body: String, _to: &'a str) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
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
