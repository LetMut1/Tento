use crate::diesel_component::schema::public::pre_confirmed_application_user as pre_confirmed_application_user_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::new::New;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, pre_confirmed_application_user: &'outer PreConfirmedApplicationUser) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(pre_confirmed_application_user_schema::table).values(New::new(pre_confirmed_application_user))
        .execute(connection_manager.get_connection()) {   // TODO нужно ли обработать количество вернувшихся строк
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn delete(connection_manager: &'outer ConnectionManager, pre_confirmed_application_user: &'outer PreConfirmedApplicationUser) -> Result<(), DieselErrorKind> {
        match diesel::delete(
            pre_confirmed_application_user_schema::table
            .filter(pre_confirmed_application_user_schema::id.eq(pre_confirmed_application_user.get_id().get_value()))
        ).execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn is_exist_by_email(connection_manager: &'outer ConnectionManager, email: &'outer str) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(
            dsl::exists(
                pre_confirmed_application_user_schema::table
                .filter(pre_confirmed_application_user_schema::email.eq(email))
            )
        ).get_result::<bool>(connection_manager.get_connection()) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(is_exist) => { return Ok(is_exist); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn get_by_email(connection_manager: &'outer ConnectionManager, email: &'outer str) -> Result<Option<PreConfirmedApplicationUser>, DieselErrorKind> {
        match pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::email.eq(email))
        .get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(existing) => {
                match existing {
                    Some(existing) => { return Ok(Some(PreConfirmedApplicationUser::new_from_model(existing))); },
                    None => { return Ok(None); }
                };
            },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }
}