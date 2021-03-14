use crate::diesel_component::schema::public::pre_registered_application_user_registration_confirmation_token as pre_registered_application_user_registration_confirmation_token_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::pre_registered_application_user_registration_confirmation_token::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::pre_registered_application_user_registration_confirmation_token::_new_for_context::new::New;
use crate::entity::entity::pre_registered_application_user_registration_confirmation_token::pre_registered_application_user_registration_confirmation_token::PreRegisteredApplicationUserRegistrationConfirmationToken;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn create(
        connection_manager: &'outer ConnectionManager, 
        pre_registered_application_user_registration_confirmation_token: &'outer PreRegisteredApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(pre_registered_application_user_registration_confirmation_token_schema::table)
        .values(New::new(pre_registered_application_user_registration_confirmation_token))
        .execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn delete(
        connection_manager: &'outer ConnectionManager, 
        pre_registered_application_user_registration_confirmation_token: &'outer PreRegisteredApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), DieselErrorKind> {
        match diesel::delete(
            pre_registered_application_user_registration_confirmation_token_schema::table
            .filter(pre_registered_application_user_registration_confirmation_token_schema::id.eq(pre_registered_application_user_registration_confirmation_token.get_id().get_value()))
        ).execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn get_by_pre_confirmed_application_user_id(
        connection_manager: &'outer ConnectionManager, pre_confirmed_application_user_id: &'outer Uuid
    ) -> Result<Option<PreRegisteredApplicationUserRegistrationConfirmationToken<'outer>>, DieselErrorKind> {
        match pre_registered_application_user_registration_confirmation_token_schema::table.filter(
            pre_registered_application_user_registration_confirmation_token_schema::pre_confirmed_application_user_id.eq(pre_confirmed_application_user_id)
        ).get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(existing) => { 
                match existing {
                    Some(existing) => { return Ok(Some(PreRegisteredApplicationUserRegistrationConfirmationToken::new_from_model(existing))); },
                    None => { return Ok(None); }
                };
             },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }
}