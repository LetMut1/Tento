use crate::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::existing::Existing;
use crate::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::new::New;
use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::pg::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn save(pg_connection: &'outer PgConnection, new: &'outer New) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user::table).values(new).execute(pg_connection) {
            Ok(_value) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn is_exist_by_nickanme(pg_connection: &'outer PgConnection, nickname: &'outer String) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(dsl::exists(application_user::table.filter(application_user::nickname.eq(nickname)))).get_result::<bool>(pg_connection) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(value) => { return Ok(value); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn is_exist_by_email(pg_connection: &'outer PgConnection, email: &'outer String) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(dsl::exists(application_user::table.filter(application_user::email.eq(email)))).get_result::<bool>(pg_connection) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(value) => { return Ok(value); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn get_by_email(pg_connection: &'outer PgConnection, email: &'outer String) -> Result<Existing, DieselErrorKind> {
        match application_user::table.filter(application_user::email.eq(email)).limit(1).load::<Existing>(pg_connection) { // TODO если вернется ноль значений, то что делать
            Ok(ref mut value) => { return Ok(value.pop().unwrap()); },  // TODO
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn delete(pg_connection: &'outer PgConnection, application_user: &'outer ApplicationUser<'outer>) -> Result<(), DieselErrorKind> {
        match diesel::delete(application_user::table.filter(application_user::id.eq(application_user.get_id().get_value()))).execute(pg_connection) {
            Ok(_value) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }
}