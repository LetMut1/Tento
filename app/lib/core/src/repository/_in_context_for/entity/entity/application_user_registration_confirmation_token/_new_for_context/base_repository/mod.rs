use crate::diesel_component::schema::public::application_user_registration_confirmation_token;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::existing::Existing;
use crate::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::new::New;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user_registration_confirmation_token::table).values(New::new(application_user_registration_confirmation_token)).execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn get_by_application_user_id(connection_manager: &'outer ConnectionManager, application_user_id: &'outer Uuid) -> Result<Option<Existing>, DieselErrorKind> {
        match application_user_registration_confirmation_token::table.filter(application_user_registration_confirmation_token::application_user_id.eq(application_user_id))
            .get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(value) => { return Ok(value); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }
}