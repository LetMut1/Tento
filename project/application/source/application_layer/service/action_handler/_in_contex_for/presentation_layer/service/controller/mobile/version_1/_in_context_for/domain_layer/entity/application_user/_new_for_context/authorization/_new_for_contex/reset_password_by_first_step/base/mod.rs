use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::value_generator::ValueGenerator;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor> {
        let application_user_email = request_data.into_inner();

        match postgresql_connection_pool.get().await {
            Ok(mut postgresql_pooled_connection) => {
                if let Some(application_user) = ApplicationUserDataProviderPostgresql::find_by_email(
                    &mut *postgresql_pooled_connection, application_user_email.as_str()
                ).await? {
                    let application_user_reset_password_token: ApplicationUserResetPasswordToken<'_>;
        
                    let application_user_id: &'_ i64;
                    match application_user.get_id() {
                        Some(application_user_id_) => {
                            application_user_id = application_user_id_;
                        }
                        None => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Application_user_id should exist")},
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
        
                    match redis_connection_pool.get().await {
                        Ok(mut redis_pooled_connection) => {
                            let redis_connection = &mut *redis_pooled_connection;

                            match ApplicationUserResetPasswordTokenDataProviderRedis::find_by_application_user_id(redis_connection, application_user_id).await? {
                                Some(application_user_reset_password_token_) => {
                                    application_user_reset_password_token = application_user_reset_password_token_;
                
                                    ApplicationUserResetPasswordTokenStateManagerRedis::update_expiration_time(redis_connection, &application_user_reset_password_token).await?;
                                }
                                None => {
                                    application_user_reset_password_token = ApplicationUserResetPasswordToken::new(
                                        application_user_id,
                                        ValueGenerator::generate(),
                                        0
                                    );
                
                                    ApplicationUserResetPasswordTokenStateManagerRedis::create(redis_connection, &application_user_reset_password_token).await?;
                                }
                            }
                
                            EmailSender::send_application_user_reset_password_token(
                                application_user_reset_password_token.get_value(), application_user.get_email()
                            )?;
                
                            return Ok(ResponseData::new(*application_user_id));
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolRedisError {bb8_redis_error: error}}},
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
        
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::NotFound}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error: error}}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}