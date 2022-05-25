use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserStateManagerPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerOutcomingData, ErrorAuditor>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_log_in_token_device_id, // TODO Это значение должно быть одно для 1 устройствоа клиента. ЛУчше сделать его постоянным, - Mac устрйоства, или что-то подобное. То значение, которе будет для КЛаудМессадж. Хранить в БД. 
            application_user_nickname,
            application_user_password,
            application_user_email,
            application_user_registration_confirmation_token_value
        )  = action_handler_incoming_data.into_inner();

        if ApplicationUserValidator::is_valid_password(application_user_password.as_str()) {
            if ApplicationUserValidator::is_valid_nickname(application_user_nickname.as_str()) {
                match postgresql_connection_pool.get().await {
                    Ok(mut postgresql_pooled_connection) => {
                        let postgresql_connection = &mut *postgresql_pooled_connection;

                        match ApplicationUserDataProviderPostgresql::is_exist_by_nickanme(postgresql_connection, application_user_nickname.as_str()).await {
                            Ok(is_exist_by_nickname) => {
                                if !is_exist_by_nickname {
                                    match ApplicationUserDataProviderPostgresql::is_exist_by_email(postgresql_connection, application_user_email.as_str()).await {
                                        Ok(is_exist_by_email) => {
                                            if !is_exist_by_email {
                                                match redis_connection_pool.get().await {
                                                    Ok(mut redis_pooled_connection) => {
                                                        let redis_connection = &mut *redis_pooled_connection;
                
                                                        match ApplicationUserRegistrationConfirmationTokenDataProviderRedis::find_by_application_user_email(
                                                            redis_connection, application_user_email.as_str()
                                                        ).await {
                                                            Ok(application_user_registration_confirmation_token) => {
                                                                if let Some(mut application_user_registration_confirmation_token_) = application_user_registration_confirmation_token {
                                                                    if application_user_registration_confirmation_token_.get_value() == application_user_registration_confirmation_token_value.as_str() {
                                                                        match PasswordHashResolver::create(application_user_password.as_str()) {
                                                                            Ok(application_user_password_hash) => {
                                                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationTokenStateManagerRedis::delete(redis_connection, &application_user_registration_confirmation_token_).await {
                                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                                    return Err(error);
                                                                                }
                                        
                                                                                let application_user = ApplicationUser::new(
                                                                                    None,
                                                                                    application_user_email,
                                                                                    application_user_nickname,
                                                                                    application_user_password_hash,
                                                                                    chrono::Utc::now().to_rfc2822() // TODO  Delete. Все Часы делаются через БД.
                                                                                );
                                                                                
                                                                                match ApplicationUserStateManagerPostgresql::create(postgresql_connection, &application_user).await {
                                                                                    Ok(application_user_id) => {
                                                                                        let json_refresh_web_token = JsonRefreshWebTokenFactory::create_from_id_registry(
                                                                                            application_user_id, application_user_log_in_token_device_id.as_str()
                                                                                        );
                                                        
                                                                                        if let Err(mut error) = RepositoryProxy::create(redis_connection, &json_refresh_web_token).await {
                                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                        
                                                                                            return Err(error);
                                                                                        }
                                                        
                                                                                        match JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token) {
                                                                                            Ok(ref json_access_web_token) => {
                                                                                                match SerializationFormResolver::serialize(environment_configuration_resolver, json_access_web_token) {
                                                                                                    Ok(json_access_web_token_) => {
                                                                                                        match Encoder::encode(environment_configuration_resolver, &json_refresh_web_token) {
                                                                                                            Ok(json_refresh_web_token_) => {
                                                                                                                return Ok(ActionHandlerOutcomingData::new(json_access_web_token_, json_refresh_web_token_));
                                                                                                            }
                                                                                                            Err(mut error) => {
                                                                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                                                                
                                                                                                                return Err(error);
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    Err(mut error) => {
                                                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                                                        
                                                                                                        return Err(error);
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(mut error) => {
                                                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                                                
                                                                                                return Err(error);
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(mut error) => {
                                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                                        
                                                                                        return Err(error);
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(mut error) => {
                                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                                
                                                                                return Err(error);
                                                                            }
                                                                        }
                                                                    }
                                        
                                                                    if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_registration_confirmation_token_) {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                        
                                                                        return Err(error);
                                                                    }
                                        
                                                                    if application_user_registration_confirmation_token_.get_wrong_enter_tries_quantity() <= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                                        if let Err(mut error) = ApplicationUserRegistrationConfirmationTokenStateManagerRedis::create(redis_connection, &application_user_registration_confirmation_token_).await {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                            return Err(error);
                                                                        }
                                                                    } else {
                                                                        if let Err(mut error) = ApplicationUserRegistrationConfirmationTokenStateManagerRedis::delete(redis_connection, &application_user_registration_confirmation_token_).await {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                            return Err(error);
                                                                        }
                                                                    }
                                                                    
                                                                    return Err(
                                                                        ErrorAuditor::new(
                                                                            ErrorAggregator::EntityError { entity_error: EntityError::ApplicationUserRegistrationConfirmationTokenError { application_user_registration_confirmation_token_error: ApplicationUserRegistrationConfirmationTokenError::InvalidValue } },
                                                                            BacktracePart::new(line!(), file!(), None)
                                                                        )
                                                                    );
                                                                }
                                        
                                                                return Err(
                                                                    ErrorAuditor::new(
                                                                        ErrorAggregator::EntityError { entity_error: EntityError::ApplicationUserRegistrationConfirmationTokenError { application_user_registration_confirmation_token_error: ApplicationUserRegistrationConfirmationTokenError::NotFound } },
                                                                        BacktracePart::new(line!(), file!(), None)
                                                                    )
                                                                );
                                                            }
                                                            Err(mut error) => {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                return Err(error);
                                                            }
                                                        }
                                                    }
                                                    Err(error) => {
                                                        return Err(
                                                            ErrorAuditor::new(
                                                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                                                                BacktracePart::new(line!(), file!(), None)
                                                            )
                                                        );
                                                    }
                                                }
                                            }
                        
                                            return Err(
                                                ErrorAuditor::new(
                                                    ErrorAggregator::EntityError { entity_error: EntityError::ApplicationUserError { application_user_error: ApplicationUserError::EmailAlreadyExist } },
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                            
                                            return Err(error);
                                        }
                                    }
                                }
                                
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::EntityError { entity_error: EntityError::ApplicationUserError { application_user_error: ApplicationUserError::NicknameAlreadyExist } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                
                                return Err(error);
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }

            return Err(
                ErrorAuditor::new(
                    ErrorAggregator::EntityError { entity_error: EntityError::ApplicationUserError { application_user_error: ApplicationUserError::InvalidNickname } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Err(
            ErrorAuditor::new(
                ErrorAggregator::EntityError { entity_error: EntityError::ApplicationUserError { application_user_error: ApplicationUserError::InvalidPassword } },
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }
}