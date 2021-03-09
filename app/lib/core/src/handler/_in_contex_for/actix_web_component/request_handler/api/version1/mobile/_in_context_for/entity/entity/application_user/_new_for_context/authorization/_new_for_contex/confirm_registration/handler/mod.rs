// use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::confirm_registration::request::Request;
// use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::confirm_registration::handler::_new_for_context::handler_result::HandlerResult;
// use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
// use crate::entity::entity::application_user::application_user::ApplicationUser;
// use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
// use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
// use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
// use crate::error::main_error_kind::main_error_kind::MainErrorKind;
// use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
// use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
// use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

// pub struct Handler;

// impl<'outer> Handler {
//     pub fn handle(request: &'outer Request) -> Result<HandlerResult, MainErrorKind> {
//         let mut connection_manager: ConnectionManager = ConnectionManager::new();
//         connection_manager.establish_connection()?;

//         match ApplicationUserBaseRepository::get_by_email(&connection_manager, request.get_email())? {
//             Some(ref value) => {
//                 let application_user: ApplicationUser<'_> = ApplicationUser::new_from_model(value);
//                 if !application_user.is_confirmed() {
//                     match ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_application_user_id(&connection_manager, application_user.get_id().get_value())? {
//                         Some(ref value) => {
//                             let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> = ApplicationUserRegistrationConfirmationToken::new_from_model(value);
//                             if !application_user_registration_confirmation_token.is_expired() {
//                                 if request.get_token() == application_user_registration_confirmation_token.get_value().get_value() {
//                                     ApplicationUserBaseRepository::save(&connection_manager, &)
//                                 } else {
//                                     connection_manager.close_connection();

//                                     return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::InvalidValue))?;
//                                 }
//                             } else {
//                                 connection_manager.close_connection();

//                                 return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::AlreadyExpired))?;
//                             }
//                         },
//                         None => {
//                             connection_manager.close_connection();

//                             return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::NotFound))?;
//                         }
//                     };
//                 } else {
//                     connection_manager.close_connection();
        
//                     return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyConfirmed))?;
//                 }
//             },
//             None => {
//                 connection_manager.close_connection();

//                 return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound))?;
//             }
//         };
//     }
// }