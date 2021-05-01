use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::redis::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::postgresql::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::entity::entity::application_user::core::email::_new_for_context::email_simple_validator::EmailSimpleValidator;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> {
        let application_user_email: Email = Email::new(request.application_user_email);

        if EmailSimpleValidator::is_valid(&application_user_email) {
            let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
            postgresql_connection_manager.establish_connection()?;

            if !PreConfirmedApplicationUserBaseRepository::is_exist_by_email(&postgresql_connection_manager, &application_user_email)? {
                if !ApplicationUserBaseRepository::is_exist_by_email(&postgresql_connection_manager, &application_user_email)? {
                    let pre_confirmed_application_user: PreConfirmedApplicationUser = PreConfirmedApplicationUser::new(application_user_email);  

                    let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> =
                    ApplicationUserRegistrationConfirmationToken::new(&pre_confirmed_application_user);

                    postgresql_connection_manager.begin_transaction()?;

                    if let Err(resource_error_kind) = PreConfirmedApplicationUserBaseRepository::create(&postgresql_connection_manager, &pre_confirmed_application_user) {
                        postgresql_connection_manager.rollback_transaction()?;

                        return Err(MainErrorKind::ResourceErrorKind(resource_error_kind));
                    }
                    
                    let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
                    redis_connection_manager.establish_connection()?;

                    if let Err(resource_error_kind) = ApplicationUserRegistrationConfirmationTokenBaseRepository::create(&mut redis_connection_manager, &application_user_registration_confirmation_token) {
                        postgresql_connection_manager.rollback_transaction()?;

                        return Err(MainErrorKind::ResourceErrorKind(resource_error_kind));
                    }
                    
                    postgresql_connection_manager.commit_transaction()?;
                    redis_connection_manager.close_connection();
                    postgresql_connection_manager.close_connection();

                    EmailSender::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

                    return Ok(());
                }
                
                return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist)));
            }
            
            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::AlreadyExist)));
        }
        
        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::InvalidEmail)));
    }
}