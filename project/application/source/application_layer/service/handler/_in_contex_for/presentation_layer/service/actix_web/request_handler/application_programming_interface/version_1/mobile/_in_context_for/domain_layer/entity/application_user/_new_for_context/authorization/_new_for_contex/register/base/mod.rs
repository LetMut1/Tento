use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedDataProviderPostgesqlTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenDataProviderRedisTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserDataProviderPostgresqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedStateManagerPostgesqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserStateManagerPostgresqlTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::wrong_enter_tries_quantity_incrementor_trait::WrongEnterTriesQuantityIncrementorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenFactoryTrait;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserPreConfirmedDataProviderPostgesql;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserPreConfirmedStateManagerPostgesql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserStateManagerPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::transaction_manager::_component::transaction_isolation_level::TransactionIsolationLevel;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::transaction_manager::transaction_manager::TransactionManager;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register::base::Base as Response;
use postgres::Client as PostgresqlConnection;
use redis::Connection as RedisConnection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<Response, BaseError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
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

        if ApplicationUserValidator::is_valid_password(application_user_password.as_str()) {
            if ApplicationUserValidator::is_valid_nickname(application_user_nickname.as_str()) {
                let postgresql_connection: &'_ mut PostgresqlConnection = &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

                if !ApplicationUserDataProviderPostgresql::is_exist_by_nickanme(postgresql_connection, application_user_nickname.as_str())? {
                    if let Some(application_user_pre_confirmed) = ApplicationUserPreConfirmedDataProviderPostgesql::find_by_application_user_email(
                        postgresql_connection, application_user_email.as_str()
                    )? 
                    {
                        let redis_connection: &'_ mut RedisConnection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                        if let Some(mut application_user_registration_confirmation_token) = ApplicationUserRegistrationConfirmationTokenDataProviderRedis::find_by_application_user_pre_confirmed_id(
                            redis_connection, application_user_pre_confirmed.get_id()?
                        )? 
                        {
                            let application_user_password_hash: String = PasswordHashResolver::create(application_user_password.as_str())?;

                            if application_user_registration_confirmation_token.get_value() == application_user_registration_confirmation_token_value.as_str() {
                                ApplicationUserRegistrationConfirmationTokenStateManagerRedis::delete(redis_connection, &application_user_registration_confirmation_token)?;

                                let transaction_manager: TransactionManager = TransactionManager::start_transaction(postgresql_connection, TransactionIsolationLevel::ReadCommitted)?;

                                if let Err(base_error) = ApplicationUserPreConfirmedStateManagerPostgesql::delete(postgresql_connection, &application_user_pre_confirmed) {
                                    transaction_manager.rollback_transaction(postgresql_connection)?;

                                    return Err(base_error);
                                }

                                let (
                                    _application_user_pre_confirmed_id,
                                    application_user_pre_confirmed_application_user_email,
                                    _application_user_pre_confirmed_created_at
                                ) : (
                                    Option<i64>,
                                    String,
                                    String
                                ) = application_user_pre_confirmed.into_inner();

                                let application_user: ApplicationUser = ApplicationUser::new(
                                    None,
                                    application_user_pre_confirmed_application_user_email,
                                    application_user_nickname,
                                    application_user_password_hash,
                                    chrono::Utc::now().to_rfc2822() // TODO  Delete. Все Часы делаются через БД.
                                );
                                
                                if let Err(base_error) = ApplicationUserStateManagerPostgresql::create(postgresql_connection, &application_user) {
                                    transaction_manager.rollback_transaction(postgresql_connection)?;

                                    return Err(base_error);
                                }
                                
                                transaction_manager.commit_transaction(postgresql_connection)?;

                                let json_refresh_web_token: JsonRefreshWebToken<'_> = JsonRefreshWebTokenFactory::create_from_id_registry(
                                    application_user.get_id()?, application_user_log_in_token_device_id.as_str()
                                );

                                RepositoryProxy::create(redis_connection, &json_refresh_web_token)?;

                                let json_access_web_token: String = SerializationFormResolver::serialize(
                                    &JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token)?
                                )?;

                                let json_refresh_web_token: String = Encoder::encode(&json_refresh_web_token)?;

                                return Ok(Response::new(json_access_web_token, json_refresh_web_token));
                            }

                            WrongEnterTriesQuantityIncrementor::increment(&mut application_user_registration_confirmation_token)?;

                            if *application_user_registration_confirmation_token.get_wrong_enter_tries_quantity() >= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                ApplicationUserRegistrationConfirmationTokenStateManagerRedis::delete(redis_connection, &application_user_registration_confirmation_token)?;
                            }
                            
                            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserRegistrationConfirmationTokenError {application_user_registration_confirmation_token_error: ApplicationUserRegistrationConfirmationTokenError::InvalidValue}});
                        }

                        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserRegistrationConfirmationTokenError {application_user_registration_confirmation_token_error: ApplicationUserRegistrationConfirmationTokenError::NotFound}});
                    }

                    if ApplicationUserDataProviderPostgresql::is_exist_by_email(postgresql_connection, application_user_email.as_str())? {
                        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserPreConfirmedError {application_user_pre_confirmed_error: ApplicationUserPreConfirmedError::AlreadyConfirmed}});
                    }
                    
                    return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserPreConfirmedError {application_user_pre_confirmed_error: ApplicationUserPreConfirmedError::NotFound}});
                }
                
                return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::AlreadyExist}});
            }

            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidNickname}});
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidPassword}});
    }
}