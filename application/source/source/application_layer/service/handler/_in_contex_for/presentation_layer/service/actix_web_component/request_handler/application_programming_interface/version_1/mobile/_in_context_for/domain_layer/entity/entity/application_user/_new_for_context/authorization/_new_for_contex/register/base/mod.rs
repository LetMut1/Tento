use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::nickname::Nickname;
use crate::domain_layer::entity::entity::application_user::_component::password::Password;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPreConfirmedPostgesqlTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserRegistrationConfirmationTokenRedisTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPostgresqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserPreConfirmedPostgesqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserRegistrationConfirmationTokenRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserPostgresqlTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::domain_layer::service::component_validator::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::base::Base as ApplicationUserComponentValidator;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::base::Base as ApplicationUserFactory;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderApplicationUserPreConfirmedPostgesql;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as DataProviderApplicationUserRegistrationConfirmationTokenRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderApplicationUserPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as StateManagerApplicationUserPreConfirmedPostgesql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as StateManagerApplicationUserRegistrationConfirmationTokenRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as StateManagerApplicationUserPostgresql;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::_resource::postgresql::_new_for_context::transaction_manager::TransactionManager;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::base::Base as Response;
use diesel::PgConnection as PostgresqlConnection;
use redis::Connection as RedisConnection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_log_in_token_device_id, 
            application_user_nickname,
            application_user_password,
            application_user_email,
            application_user_registration_confirmation_token_value
        ) : (
            String,
            String,
            String,
            String,
            String
        ) = request.into_inner();

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId = ApplicationUserLogInTokenDeviceId::new_from_string(
            application_user_log_in_token_device_id
        )?;

        let application_user_nickname: Nickname = Nickname::new(application_user_nickname);

        let application_user_email: Email = Email::new(application_user_email);

        let application_user_password: Password = Password::new(application_user_password);
        if ApplicationUserComponentValidator::is_valid_password(&application_user_password) {
            if ApplicationUserComponentValidator::is_valid_nickname(&application_user_nickname) {
                let postgresql_connection: &'_ PostgresqlConnection = &*ConnectionExtractor::get_postgresqlxxxdelete_connection(&aggregate_connection_pool)?;

                if !DataProviderApplicationUserPostgresql::is_exist_by_nickanme(postgresql_connection, &application_user_nickname)? {
                    if let Some(application_user_pre_confirmed) = DataProviderApplicationUserPreConfirmedPostgesql::get_by_application_user_email(postgresql_connection, &application_user_email)? {
                        let redis_connection: &'_ mut RedisConnection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                        if let Some(mut application_user_registration_confirmation_token) = DataProviderApplicationUserRegistrationConfirmationTokenRedis::get_by_application_user_pre_confirmed_id(
                            redis_connection, application_user_pre_confirmed.get_id()?
                        )? 
                        {
                            if application_user_registration_confirmation_token.get_value().get_value() == application_user_registration_confirmation_token_value.as_str() {
                                let application_user: ApplicationUser<'_> = ApplicationUserFactory::new_from_application_user_pre_confirmed(
                                    &application_user_pre_confirmed, application_user_nickname, PasswordHashResolver::create(&application_user_password)?
                                );

                                StateManagerApplicationUserRegistrationConfirmationTokenRedis::delete(redis_connection, &application_user_registration_confirmation_token)?;

                                TransactionManager::begin_transaction(postgresql_connection)?;
                                
                                if let Err(base_error) = StateManagerApplicationUserPostgresql::create(postgresql_connection, &application_user) {
                                    TransactionManager::rollback_transaction(postgresql_connection)?;

                                    return Err(base_error);
                                }

                                if let Err(base_error) = StateManagerApplicationUserPreConfirmedPostgesql::delete(postgresql_connection, &application_user_pre_confirmed) {
                                    TransactionManager::rollback_transaction(postgresql_connection)?;

                                    return Err(base_error);
                                }
                                
                                TransactionManager::commit_transaction(postgresql_connection)?;

                                let json_refresh_web_token: JsonRefreshWebToken<'_> = JsonRefreshWebTokenFactory::new_from_id_registry(application_user.get_id()?, &application_user_log_in_token_device_id);

                                RepositoryProxy::create(redis_connection, &json_refresh_web_token)?;

                                let json_access_web_token: String = SerializationFormResolver::serialize(&JsonAccessWebTokenFactory::new_from_json_refresh_web_token(&json_refresh_web_token)?)?;

                                let json_refresh_web_token: String = Encoder::encode(&json_refresh_web_token)?;

                                return Ok(Response::new(json_access_web_token, json_refresh_web_token));
                            }

                            application_user_registration_confirmation_token.increment_wrong_enter_tries_quantity();

                            if application_user_registration_confirmation_token.get_wrong_enter_tries_quantity().get_value() >= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                StateManagerApplicationUserRegistrationConfirmationTokenRedis::delete(redis_connection, &application_user_registration_confirmation_token)?;
                            }
                            
                            return Err(BaseError::EntityError(EntityError::ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError::InvalidValue)));
                        }

                        return Err(BaseError::EntityError(EntityError::ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError::NotFound)));
                    }

                    if DataProviderApplicationUserPostgresql::is_exist_by_email(postgresql_connection, &application_user_email)? {
                        return Err(BaseError::EntityError(EntityError::ApplicationUserPreConfirmedError(ApplicationUserPreConfirmedError::AlreadyConfirmed)));
                    }
                    
                    return Err(BaseError::EntityError(EntityError::ApplicationUserPreConfirmedError(ApplicationUserPreConfirmedError::NotFound)));
                }
                
                return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::AlreadyExist)));
            }

            return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::InvalidNickname)));
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::InvalidPassword)));
    }
}