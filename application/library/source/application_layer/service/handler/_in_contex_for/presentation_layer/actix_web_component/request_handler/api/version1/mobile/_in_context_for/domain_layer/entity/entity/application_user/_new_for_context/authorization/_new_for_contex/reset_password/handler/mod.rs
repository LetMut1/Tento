use crate::domain_layer::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::entity_error::EntityError;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::domain_layer::utility::_in_context_for::data_transfer_object::resource_model::_new_for_context::update_resolver::_in_context_for::_in_context_for::entity::entity::application_user::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password::request::Request;
use diesel::PgConnection as PostgresqlConnection;
use redis::Connection as RedisConnection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<(), BaseError> { // TODO  !!!!! Это ресет для пользователя, забывшего пароль. НО также нужно сделать АККУРАТНО ресетпассворд для залогиневшегося пользователя с повторной отправкой старого пароля !!!!!!!!!
        let (
            application_user_id, 
            application_user_password,
            application_user_reset_password_token_value
        ) = request.into_inner();

        let application_user_password: Password = Password::new(application_user_password);
        if application_user_password.is_valid() {
            let application_user_id: ApplicationUserId = ApplicationUserId::new(application_user_id);
            
            let redis_connection: &'_ mut RedisConnection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

            if let Some(mut application_user_reset_password_token) = ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(
                redis_connection, &application_user_id
            )? 
            {
                if application_user_reset_password_token.get_value().get_value() == application_user_reset_password_token_value.as_str() {
                    let postgresql_connection: &'_ PostgresqlConnection = &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

                    if let Some(mut application_user) = ApplicationUserBaseRepository::get_by_id(postgresql_connection, &application_user_id)? {
                        application_user.set_password(application_user_password)?;

                        ApplicationUserBaseRepository::update(postgresql_connection, &application_user, UpdateResolver::new(false, false, true, false))?; // TODO Загуглить, чтл можно сделать для обеспечения транзакции на две системы (зкроме, запоминания состояния через третью ссистпму)

                        ApplicationUserResetPasswordTokenBaseRepository::delete(redis_connection, &application_user_reset_password_token)?;

                        return Ok(());
                    }

                    return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::NotFound)));
                }

                application_user_reset_password_token.increment_wrong_enter_tries_quantity();

                if application_user_reset_password_token.get_wrong_enter_tries_quantity().get_value() >= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                    ApplicationUserResetPasswordTokenBaseRepository::delete(redis_connection, &application_user_reset_password_token)?;
                }


                return Err(BaseError::EntityError(EntityError::ApplicationUserResetPasswordTokenError(ApplicationUserResetPasswordTokenError::InvalidValue)));
            }

            return Err(BaseError::EntityError(EntityError::ApplicationUserResetPasswordTokenError(ApplicationUserResetPasswordTokenError::NotFound)));
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::InvalidPassword)));
    }
}