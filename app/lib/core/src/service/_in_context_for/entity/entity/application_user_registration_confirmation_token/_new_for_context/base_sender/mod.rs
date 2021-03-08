// use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
// use crate::entity::entity::application_user::core::email::Email;
// use crate::utility::email_sender::EmailSender;

// pub struct BaseSender;

// impl<'outer> BaseSender {
//     pub fn send_by_email(
//         application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken, 
//         email: &'outer Email<'outer>
//     ) -> () {
//         EmailSender::send(
//             "Registration confirmation".to_string(),
//              "Your code: ".to_string() + application_user_registration_confirmation_token.get_value().get_value(),
//               email.get_value().clone()
//         );  

//         return ();
//     }
// }