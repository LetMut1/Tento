use crate::diesel_component::model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::new::New;
use crate::diesel_component::schema::public::application_user_registration_confirmation_token;
use crate::error::context::Context;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn save(pg_connection_manager: &'outer PgConnection, new: &'outer New) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user_registration_confirmation_token::table).values(new).execute(pg_connection_manager) {
            Ok(_value) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::Any(Context::new(Some(value), None))); }
        };
    }
}