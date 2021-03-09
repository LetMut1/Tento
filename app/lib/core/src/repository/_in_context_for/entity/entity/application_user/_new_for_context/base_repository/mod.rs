use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::existing::Existing;
use crate::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::new::New;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, application_user: &'outer ApplicationUser<'outer>) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user::table).values(New::new(application_user)).execute(connection_manager.get_connection()) {   // TODO нужно ли обработать количество вернувшихся строк
            Ok(_) => { return Ok(()); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn is_exist_by_nickanme(connection_manager: &'outer ConnectionManager, nickname: &'outer String) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(dsl::exists(application_user::table.filter(application_user::nickname.eq(nickname)))).get_result::<bool>(connection_manager.get_connection()) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(value) => { return Ok(value); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn is_exist_by_email(connection_manager: &'outer ConnectionManager, email: &'outer String) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(dsl::exists(application_user::table.filter(application_user::email.eq(email)))).get_result::<bool>(connection_manager.get_connection()) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(value) => { return Ok(value); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    pub fn get_by_email(connection_manager: &'outer ConnectionManager, email: &'outer String) -> Result<Option<Existing>, DieselErrorKind> {
        match application_user::table.filter(application_user::email.eq(email)).get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(value) => { return Ok(value); },
            Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
        };
    }

    // pub fn delete(connection_manager: &'outer ConnectionManager, application_user: &'outer ApplicationUser<'outer>) -> Result<(), DieselErrorKind> {
    //     match diesel::delete(application_user::table.filter(application_user::id.eq(application_user.get_id().get_value()))).execute(connection_manager.get_connection()) {
    //         Ok(_) => { return Ok(()); },
    //         Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
    //     };
    // }
}