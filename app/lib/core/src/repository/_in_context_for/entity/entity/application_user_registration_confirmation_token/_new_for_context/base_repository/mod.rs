use crate::diesel_component::schema::public::application_user_registration_confirmation_token;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::new::New;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn save(connection_manager: &'outer ConnectionManager, new: &'outer New) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user_registration_confirmation_token::table).values(new).execute(connection_manager.get_connection()) {
            Ok(_value) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }
}