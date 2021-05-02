use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password::request::Request;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::core::password::Password;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error_kind::ApplicationUserResetPasswordTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::redis::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::utility::_in_context_for::data_transfer_object::resource_model::_new_for_context::update_resolver::_in_context_for::_in_context_for::entity::entity::application_user::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> {
        let application_user_id: UuidV4 = UuidV4::new_from_string(request.application_user_id)?;

        let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
        redis_connection_manager.establish_connection()?;

        if let Some(application_user_reset_password_token) = ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(&mut redis_connection_manager, &application_user_id)? {
            if application_user_reset_password_token.get_value().get_value() == request.application_user_reset_password_token_value.as_str() {  // TODO переписать через НЕ
                let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
                postgresql_connection_manager.establish_connection()?;

                if let Some(mut application_user) = ApplicationUserBaseRepository::get_by_id(&postgresql_connection_manager, &application_user_id)? {
                    application_user.set_password(Password::new(request.application_user_password));

                    ApplicationUserBaseRepository::update(&postgresql_connection_manager, &application_user, UpdateResolver::new(false, false, true, false))?;

                    postgresql_connection_manager.close_connection();

                    ApplicationUserResetPasswordTokenBaseRepository::delete(&mut redis_connection_manager, &application_user_reset_password_token)?;

                    redis_connection_manager.close_connection();

                    return Ok(());
                }

                return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound)));
            }

            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserResetPasswordTokenErrorKind(ApplicationUserResetPasswordTokenErrorKind::InvalidValue)));
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserResetPasswordTokenErrorKind(ApplicationUserResetPasswordTokenErrorKind::NotFound)));
    }
}