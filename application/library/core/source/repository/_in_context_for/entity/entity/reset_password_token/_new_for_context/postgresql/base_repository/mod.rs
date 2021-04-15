use crate::diesel_component::schema::public::reset_password_token as reset_password_token_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::reset_password_token::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::reset_password_token::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::reset_password_token::reset_password_token::ResetPasswordToken;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer ConnectionManager, 
        reset_password_token: &'outer ResetPasswordToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::insert_into(reset_password_token_schema::table)
        .values(New::new(reset_password_token))
        .execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn update(
        connection_manager: &'outer ConnectionManager,
        reset_password_token: &'outer ResetPasswordToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::update(
            reset_password_token_schema::table
            .filter(reset_password_token_schema::id.eq(reset_password_token.get_id().get_value()))
        ).set(
            reset_password_token_schema::expired_at.eq(reset_password_token.get_expired_at().get_value())
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn delete(
        connection_manager: &'outer ConnectionManager, 
        reset_password_token: &'outer ResetPasswordToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::delete(
            reset_password_token_schema::table
            .filter(reset_password_token_schema::id.eq(reset_password_token.get_id().get_value()))
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn get_by_application_user_id(
        connection_manager: &'outer ConnectionManager, application_user_id: &'outer UuidV4
    ) -> Result<Option<ResetPasswordToken<'vague>>, DieselError> {
        if let Some(existing) = reset_password_token_schema::table
        .filter(reset_password_token_schema::application_user_id.eq(application_user_id.get_value()))
        .get_result::<Existing>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ResetPasswordToken::new_from_model(existing))); 
        }

        return Ok(None);
    }
}