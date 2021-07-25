use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::application_user::_core::nickname::Nickname;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error::PreConfirmedApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::entity_error::EntityError;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::validator::Validator;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy_trait::BaseRepositoryProxyTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::_resource::postgresql::_new_for_context::transaction_manager::TransactionManager;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::response::Response;
use diesel::PgConnection as PostgresqlConnection;
use redis::Connection as RedisConnection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_log_in_token_device_id, 
            application_user_nickname,
            application_user_password,
            application_user_email,
            application_user_registration_confirmation_token_value
        ) = request.into_inner();

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId = ApplicationUserLogInTokenDeviceId::new_from_string(
            application_user_log_in_token_device_id
        )?;

        let application_user_nickname: Nickname = Nickname::new(application_user_nickname);

        let application_user_email: Email = Email::new(application_user_email);

        let application_user_password: Password = Password::new(application_user_password);
        if Validator::is_valid_password(&application_user_password) {
            let postgresql_connection: &'_ PostgresqlConnection = &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

            if !ApplicationUserBaseRepository::is_exist_by_nickanme(postgresql_connection, &application_user_nickname)? {
                if let Some(pre_confirmed_application_user) = PreConfirmedApplicationUserBaseRepository::get_by_application_user_email(postgresql_connection, &application_user_email)? {
                    let redis_connection: &'_ mut RedisConnection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                    if let Some(mut application_user_registration_confirmation_token) = ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user_id(
                        redis_connection, pre_confirmed_application_user.get_id()?
                    )? 
                    {
                        if application_user_registration_confirmation_token.get_value().get_value() == application_user_registration_confirmation_token_value.as_str() {
                            let application_user: ApplicationUser<'_> = ApplicationUser::new_from_pre_confirmed_application_user(
                                &pre_confirmed_application_user, application_user_nickname, PasswordHashResolver::create(&application_user_password)?
                            );

                            ApplicationUserRegistrationConfirmationTokenBaseRepository::delete(redis_connection, &application_user_registration_confirmation_token)?;

                            TransactionManager::begin_transaction(postgresql_connection)?;
                            
                            if let Err(base_error) = ApplicationUserBaseRepository::create(postgresql_connection, &application_user) {
                                TransactionManager::rollback_transaction(postgresql_connection)?;

                                return Err(base_error);
                            }

                            if let Err(base_error) = PreConfirmedApplicationUserBaseRepository::delete(postgresql_connection, &pre_confirmed_application_user) {
                                TransactionManager::rollback_transaction(postgresql_connection)?;

                                return Err(base_error);
                            }
                            
                            TransactionManager::commit_transaction(postgresql_connection)?;

                            let json_refresh_web_token: JsonRefreshWebToken<'_> = JsonRefreshWebToken::new(application_user.get_id()?, &application_user_log_in_token_device_id);

                            <BaseRepositoryProxy as BaseRepositoryProxyTrait>::create(redis_connection, &json_refresh_web_token)?;

                            return Ok(
                                Response::new(
                                    <SerializationFormResolver as SerializationFormResolverTrait>::serialize(&JsonAccessWebToken::new(&json_refresh_web_token)?)?,
                                    <Encoder as EncoderTrait>::encode(&json_refresh_web_token)?
                                )
                            );
                        }

                        application_user_registration_confirmation_token.increment_wrong_enter_tries_quantity();

                        if application_user_registration_confirmation_token.get_wrong_enter_tries_quantity().get_value() >= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                            ApplicationUserRegistrationConfirmationTokenBaseRepository::delete(redis_connection, &application_user_registration_confirmation_token)?;
                        }
                        
                        return Err(BaseError::EntityError(EntityError::ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError::InvalidValue)));
                    }

                    return Err(BaseError::EntityError(EntityError::ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError::NotFound)));
                }

                if ApplicationUserBaseRepository::is_exist_by_email(postgresql_connection, &application_user_email)? {
                    return Err(BaseError::EntityError(EntityError::PreConfirmedApplicationUserError(PreConfirmedApplicationUserError::AlreadyConfirmed)));
                }
                
                return Err(BaseError::EntityError(EntityError::PreConfirmedApplicationUserError(PreConfirmedApplicationUserError::NotFound)));
            }
            
            return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::AlreadyExist)));
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::InvalidPassword)));
    }
}