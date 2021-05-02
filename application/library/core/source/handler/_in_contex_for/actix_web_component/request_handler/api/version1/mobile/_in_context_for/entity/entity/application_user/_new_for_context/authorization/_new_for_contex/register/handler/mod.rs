use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::core::password::Password;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::redis::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::postgresql::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl<'outer_a> Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let application_user_nickname: Nickname = Nickname::new(request.application_user_nickname);

        let application_user_email: Email = Email::new(request.application_user_email);

        let application_user_log_in_token_device_id: UuidV4 = UuidV4::new_from_string(request.application_user_log_in_token_device_id)?;

        let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
        postgresql_connection_manager.establish_connection()?;

        if !ApplicationUserBaseRepository::is_exist_by_nickanme(&postgresql_connection_manager, &application_user_nickname)? {
            if let Some(pre_confirmed_application_user) = PreConfirmedApplicationUserBaseRepository::get_by_email(&postgresql_connection_manager, &application_user_email)? {
                let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
                redis_connection_manager.establish_connection()?;

                if let Some(application_user_registration_confirmation_token) = 
                ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user_id(&mut redis_connection_manager, pre_confirmed_application_user.get_id())? 
                {
                    if request.application_user_registration_confirmation_token_value.as_str() == application_user_registration_confirmation_token.get_value().get_value() {
                        let application_user: ApplicationUser<'_> = 
                        ApplicationUser::new_from_pre_confirmed_application_user(&pre_confirmed_application_user, application_user_nickname, Password::new(request.application_user_password));

                        ApplicationUserRegistrationConfirmationTokenBaseRepository::delete(&mut redis_connection_manager, &application_user_registration_confirmation_token)?;

                        postgresql_connection_manager.begin_transaction()?;
                        
                        if let Err(resource_error_kind) = ApplicationUserBaseRepository::create(&postgresql_connection_manager, &application_user) {
                            postgresql_connection_manager.rollback_transaction()?;

                            return Err(MainErrorKind::ResourceErrorKind(resource_error_kind));
                        }

                        if let Err(resource_error_kind) = PreConfirmedApplicationUserBaseRepository::delete(&postgresql_connection_manager, &pre_confirmed_application_user) {
                            postgresql_connection_manager.rollback_transaction()?;

                            return Err(MainErrorKind::ResourceErrorKind(resource_error_kind));
                        }
                        
                        postgresql_connection_manager.commit_transaction()?;

                        let json_refresh_web_token: JsonRefreshWebToken<'_> = JsonRefreshWebToken::new(application_user.get_id(), &application_user_log_in_token_device_id);

                        BaseRepositoryProxy::create(&mut redis_connection_manager, &json_refresh_web_token)?;
                        
                        redis_connection_manager.close_connection(); 

                        postgresql_connection_manager.close_connection(); 

                        return Ok(
                            HandlerResult::new(
                                SerializationFormResolver::serialize(&JsonAccessWebToken::new(&json_refresh_web_token)),
                                Encoder::encode(&json_refresh_web_token)
                            )
                        );
                    }
                    
                    return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::InvalidValue)));
                }

                return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::NotFound)));
            }

            if ApplicationUserBaseRepository::is_exist_by_email(&postgresql_connection_manager, &application_user_email)? {
                return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::AlreadyConfirmed)));
            }
            
            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::NotFound)));
        }
        
        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist)));
    }
}