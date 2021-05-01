use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::redis::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
        postgresql_connection_manager.establish_connection()?;

        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(&postgresql_connection_manager, &Email::new(request.application_user_email))? {
            let mut application_user_reset_password_token: ApplicationUserResetPasswordToken<'_>;

            let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
            redis_connection_manager.establish_connection()?;

            match ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(&mut redis_connection_manager, application_user.get_id())? {
                Some(existing_application_user_reset_password_token) => {
                    application_user_reset_password_token = existing_application_user_reset_password_token;
                    application_user_reset_password_token.refresh();

                    ApplicationUserResetPasswordTokenBaseRepository::update(&mut redis_connection_manager, &application_user_reset_password_token)?;
                },
                None => {
                    application_user_reset_password_token = ApplicationUserResetPasswordToken::new(&application_user);

                    ApplicationUserResetPasswordTokenBaseRepository::create(&mut redis_connection_manager, &application_user_reset_password_token)?;
                }
            }

            redis_connection_manager.close_connection();

            postgresql_connection_manager.close_connection();

            EmailSender::send_application_user_reset_password_token(&application_user_reset_password_token)?;

            return Ok(HandlerResult::new(application_user.get_id().get_value().to_string()));
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound)));
    }
}