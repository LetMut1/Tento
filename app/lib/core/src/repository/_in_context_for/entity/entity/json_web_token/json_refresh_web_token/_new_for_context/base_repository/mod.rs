use crate::diesel_component::schema::public::json_refresh_web_token;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::dto::resourse_model::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::new::New;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(json_refresh_web_token::table).values(New::new(json_refresh_web_token)).execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }
}