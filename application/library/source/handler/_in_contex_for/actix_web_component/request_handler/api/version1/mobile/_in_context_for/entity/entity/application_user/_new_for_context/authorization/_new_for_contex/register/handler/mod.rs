use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::core::password::Password;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error::core::entity_error::core::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::error::main_error::core::entity_error::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::error::main_error::core::entity_error::core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error::PreConfirmedApplicationUserError;
use crate::error::main_error::core::entity_error::entity_error::EntityError;
use crate::error::main_error::main_error::MainError;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::utility::_in_context_for::_resource::postgresql::_new_for_context::transaction_manager::TransactionManager;
use diesel::PgConnection as PostgresqlConnection;
use redis::Connection as RedisConnection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<HandlerResult, MainError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let application_user_nickname: Nickname = Nickname::new(request.application_user_nickname);

        let application_user_email: Email = Email::new(request.application_user_email);

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId =
        ApplicationUserLogInTokenDeviceId::new_from_string(request.application_user_log_in_token_device_id)?;

        let postgresql_connection: &'_ PostgresqlConnection = &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

        if !ApplicationUserBaseRepository::is_exist_by_nickanme(postgresql_connection, &application_user_nickname)? {
            if let Some(pre_confirmed_application_user) = PreConfirmedApplicationUserBaseRepository::get_by_application_user_email(postgresql_connection, &application_user_email)? {
                let redis_connection: &'_ mut RedisConnection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                if let Some(mut application_user_registration_confirmation_token) = 
                ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user_id(redis_connection, pre_confirmed_application_user.get_id())? 
                {
                    if request.application_user_registration_confirmation_token_value.as_str() == application_user_registration_confirmation_token.get_value().get_value() {
                        let application_user: ApplicationUser<'_> = 
                        ApplicationUser::new_from_pre_confirmed_application_user(&pre_confirmed_application_user, application_user_nickname, Password::new(request.application_user_password));

                        ApplicationUserRegistrationConfirmationTokenBaseRepository::delete(redis_connection, &application_user_registration_confirmation_token)?;

                        TransactionManager::begin_transaction(postgresql_connection)?;
                        
                        if let Err(run_time_error) = ApplicationUserBaseRepository::create(postgresql_connection, &application_user) {
                            TransactionManager::rollback_transaction(postgresql_connection)?;

                            return Err(MainError::RunTimeError(run_time_error));
                        }

                        if let Err(run_time_error) = PreConfirmedApplicationUserBaseRepository::delete(postgresql_connection, &pre_confirmed_application_user) {
                            TransactionManager::rollback_transaction(postgresql_connection)?;

                            return Err(MainError::RunTimeError(run_time_error));
                        }
                        
                        TransactionManager::commit_transaction(postgresql_connection)?;

                        let json_refresh_web_token: JsonRefreshWebToken<'_> = JsonRefreshWebToken::new(application_user.get_id(), &application_user_log_in_token_device_id);

                        BaseRepositoryProxy::create(redis_connection, &json_refresh_web_token)?;

                        return Ok(
                            HandlerResult::new(
                                SerializationFormResolver::serialize(&JsonAccessWebToken::new(&json_refresh_web_token)),
                                Encoder::encode(&json_refresh_web_token)
                            )
                        );
                    }

                    application_user_registration_confirmation_token.increment_wrong_enter_tries_quantity();

                    if application_user_registration_confirmation_token.get_wrong_enter_tries_quantity().get_value() >= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                        ApplicationUserRegistrationConfirmationTokenBaseRepository::delete(redis_connection, &application_user_registration_confirmation_token)?;
                    }
                    
                    return Err(MainError::EntityError(EntityError::ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError::InvalidValue)));
                }

                return Err(MainError::EntityError(EntityError::ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError::NotFound)));
            }

            if ApplicationUserBaseRepository::is_exist_by_email(postgresql_connection, &application_user_email)? {
                return Err(MainError::EntityError(EntityError::PreConfirmedApplicationUserError(PreConfirmedApplicationUserError::AlreadyConfirmed)));
            }
            
            return Err(MainError::EntityError(EntityError::PreConfirmedApplicationUserError(PreConfirmedApplicationUserError::NotFound)));
        }
        
        return Err(MainError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::AlreadyExist)));
    }
}